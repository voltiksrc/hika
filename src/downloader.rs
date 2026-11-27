// SPDX-License-Identifier: GPL-3.0-only

use std::{error::Error, fs, io, path::Path, time::Duration};

use crate::{checksum::verify_sha256, output, recipe::Artifact};

// Handles the file transfer independently of verification and finalization.
fn fetch(client: &reqwest::blocking::Client, url: &str, temp: &Path) -> Result<(), Box<dyn Error>> {
    let url = reqwest::Url::parse(url).map_err(|err| format!("Invalid URL: {err}"))?;

    // Reject unencrypted connections before sending a request.
    if url.scheme() != "https" {
        return Err("Only HTTPS URLs are supported".into());
    }

    let mut response = client.get(url).send()?.error_for_status()?;

    // create_new prevents accidentally overwriting an existing temporary file.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp)?;

    // Stream the response directly to disk instead of storing the entire
    // archive in memory, which could be expensive for large packages.
    if let Err(err) = io::copy(&mut response, &mut file) {
        drop(file);
        let _ = fs::remove_file(temp);
        return Err(err.into());
    }

    // Flush file contents and metadata to disk before verification.
    if let Err(err) = file.sync_all() {
        drop(file);
        let _ = fs::remove_file(temp);
        return Err(err.into());
    }

    Ok(())
}

pub fn download_from_url(
    client: &reqwest::blocking::Client,
    url: &str,
    destination: &Path,
    expected_hash: &str,
) -> Result<(), Box<dyn Error>> {
    let temp = destination.with_added_extension("tmp");

    output::begin("Downloading archive");

    let result = fetch(client, url, &temp);

    // Finish the status line before propagating any transfer errors.
    output::end(result.is_ok());
    result?;

    output::begin("Verifying sha256");

    let result = verify_sha256(&temp, expected_hash);

    output::end(result.is_ok());

    // Discard invalid archives so the next mirror can retry the download.
    if let Err(err) = result {
        let _ = fs::remove_file(&temp);
        return Err(err);
    }

    // Only expose the archive at its final path after verification succeeds.
    if let Err(err) = fs::rename(&temp, destination) {
        let _ = fs::remove_file(&temp);
        return Err(err.into());
    }

    Ok(())
}

pub fn download(artifact: &Artifact, destination: &Path) -> Result<(), Box<dyn Error>> {
    let temp = destination.with_added_extension("tmp");

    // Avoid overwriting existing archives or leftover temporary downloads.
    if temp.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "temporary file already exists",
        )
        .into());
    }

    if destination.exists() {
        output::info("Found cached archive");
        output::begin("Verifying sha256");

        let result = verify_sha256(destination, &artifact.sha256);

        output::end(result.is_ok());

        match result {
            Ok(()) => return Ok(()),
            Err(err) => {
                output::warn(&format!("Cached archive verification failed: {err}"));
                fs::remove_file(destination)?;
            }
        }
    }

    // Reuse the same HTTP client across all mirror attempts.
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(120))
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            // Prevent redirect loops and HTTPS-to-HTTP downgrades.
            if attempt.previous().len() >= 10 {
                attempt.error("Too many redirects")
            } else if attempt.url().scheme() != "https" {
                attempt.error("Redirect to non-HTTPS URL rejected")
            } else {
                attempt.follow()
            }
        }))
        .build()?;

    let mut errors = Vec::new();

    // Try mirrors in recipe order until an archive is successfully
    // downloaded, verified and moved to its final destination.
    for url in &artifact.urls {
        match download_from_url(&client, url, destination, &artifact.sha256) {
            Ok(()) => return Ok(()),
            Err(err) => {
                errors.push(format!("Mirror {url} failed: {err}"));
            }
        }
    }

    // Preserve errors from every mirror to make download failures
    // easier to diagnose.
    Err(format!("All download mirrors failed\n  {}", errors.join("\n  ")).into())
}
