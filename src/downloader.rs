// SPDX-License-Identifier: GPL-3.0-only

use std::{error::Error, fs, io, path::Path, time::Duration};

use crate::checksum::verify_sha256;
use crate::recipe::Artifact;

pub fn download_from_url(
    client: &reqwest::blocking::Client,
    url: &str,
    destination: &Path,
    expected_hash: &str,
) -> Result<(), Box<dyn Error>> {
    let temp = destination.with_added_extension("tmp");

    // Validate the URL before attempting a download.
    let url = reqwest::Url::parse(url).map_err(|err| format!("Invalid URL: {err}"))?;

    if url.scheme() != "https" {
        return Err("Only HTTPS URLs are supported".into());
    }

    let mut response = client.get(url).send()?.error_for_status()?;

    // Write to a temp file so incomplete downloads aren't treated as finished.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;

    if let Err(err) = io::copy(&mut response, &mut file) {
        drop(file);
        let _ = fs::remove_file(&temp);
        return Err(err.into());
    }

    // Make sure everything is written before moving it to its final path.
    if let Err(err) = file.sync_all() {
        drop(file);
        let _ = fs::remove_file(&temp);
        return Err(err.into());
    }

    drop(file);
    if let Err(err) = verify_sha256(&temp, expected_hash) {
        let _ = fs::remove_file(&temp);
        return Err(err);
    }

    // If it fails to rename the temporary file, remove it.
    if let Err(err) = fs::rename(&temp, destination) {
        let _ = fs::remove_file(&temp);
        return Err(err.into());
    }

    Ok(())
}

pub fn download(artifact: &Artifact, destination: &Path) -> Result<(), Box<dyn Error>> {
    let temp = destination.with_added_extension("tmp");

    // Don't overwrite an existing download or leftover temp file.
    if destination.exists() || temp.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Output or temporary file already exists",
        )
        .into());
    }

    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(120))
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
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

    for url in &artifact.urls {
        match download_from_url(&client, url, destination, &artifact.sha256) {
            Ok(()) => return Ok(()),
            Err(err) => {
                errors.push(format!("Mirror {url} failed: {err}"));
            }
        }
    }

    Err(format!("All download mirrors failed\n  {}", errors.join("\n  ")).into())
}
