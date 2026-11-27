// SPDX-License-Identifier: GPL-3.0-only

use std::{error::Error, fs};
use tempfile::Builder;

use crate::{archive, downloader, output, paths, recipe};

pub fn acquire_package(name: &str) -> Result<(), Box<dyn Error>> {
    let path = paths::recipe(name);

    output::begin(&format!("Loading recipe for {name}"));

    let result = recipe::load_recipe(&path);

    output::end(result.is_ok());

    let package = result?;

    output::info(&format!(
        "Loaded recipe for {} {}",
        package.name, package.version
    ));

    let artifact = package
        .artifacts
        .first()
        .ok_or("Package has no artifacts")?;

    let url = artifact
        .urls
        .first()
        .ok_or("Artifact has no download URLs")?;

    output::info(&format!("Selected download URL: {url}"));

    let parsed = url::Url::parse(url)?;

    // Get the filename from the url rather than hardcoding it
    let filename = parsed
        .path_segments()
        .and_then(|mut segments| segments.next_back())
        .filter(|name| !name.is_empty())
        .ok_or("URL has no filename")?;

    let destination = paths::downloads().join(filename);

    fs::create_dir_all(paths::downloads())?;

    downloader::download(artifact, &destination)?;

    fs::create_dir_all("cache/staging")?;

    // Create a unique staging dir to account for stale staging directories and files
    let staging = Builder::new().prefix("hika-").tempdir_in("cache/staging")?;

    output::begin(&format!("Extracting {}-{}", package.name, package.version));

    let result = archive::extract(&destination, staging.path());

    output::end(result.is_ok());

    result?;

    // TODO: Check file conflicts.
    // TODO: Merge into target root.
    // TODO: Register package in database.

    Ok(())
}
