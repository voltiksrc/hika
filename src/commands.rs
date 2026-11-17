// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;

use crate::{downloader, output, recipe};

pub fn acquire(packages: Vec<String>) {
    for package in packages {
        let path = format!("recipe/{package}/package.lua");

        // Load the recipe before doing anything with the package.
        output::begin(&format!("Loading recipe for {package}"));

        match recipe::load_recipe(Path::new(&path)) {
            Ok(package) => {
                output::end(true);
                output::info(&format!("Loaded {} {}", package.name, package.version));

                // Temporarily add some printing
                output::info(&format!("Description: {}", package.description));
                output::info(&format!("Runtime dependencies: {:?}", package.runtime_deps));
                output::info(&format!("Artifacts: {}", package.artifacts.len()));

                // Use the first artifact and its primary download URL for now.
                if let Some(artifact) = package.artifacts.first()
                    && let Some(url) = artifact.urls.first()
                {
                    output::info(&format!("Selected download URL: {url}"));

                    let filename = format!("{}-{}.deb", package.name, package.version);
                    let destination = Path::new("downloads").join(filename);

                    if let Err(err) = std::fs::create_dir_all("downloads") {
                        output::error(&format!("Failed to create downloads directory: {err}"));
                        continue;
                    }

                    output::begin(&format!("Downloading {}", package.name));

                    match downloader::download(artifact, &destination) {
                        Ok(()) => output::end(true),
                        Err(err) => {
                            output::end(false);
                            output::error(&format!("Download failed: {err}"));
                        }
                    }
                }

                //output::info(&format!("Acquiring {}", package.name));
            }
            Err(err) => {
                output::end(false);
                output::error(&format!("Failed to load {package}: {err}"));
                continue;
            }
        }
    }
}

pub fn reacquire(packages: Vec<String>) {
    for package in packages {
        output::info(&format!("Reacquiring {package}"));
    }
}

pub fn transcend() {
    // Package updates will go here once the backend is ready.
    output::info("Transcending all packages...");
}
