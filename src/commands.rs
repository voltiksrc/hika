// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;

use crate::{
    downloader, garden,
    garden::reconcile::{self, Change},
    output, recipe,
};

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

pub fn reconcile() {
    let path = Path::new("/etc/garden/system.lua");

    output::begin("Reading garden configuration");

    match garden::load(path) {
        Ok(config) => {
            output::end(true);
            match reconcile::compare_hostname(&config) {
                Ok(Some(Change::SetHostname { old, new })) => {
                    println!("  ~ Hostname: {old} → {new}");
                }
                Ok(None) => {
                    output::info("Hostname is already correct");
                }
                Err(err) => {
                    output::error(&format!("Failed to compare hostname: {err}"));
                }
            }

            // Print the configuration after it's finished loading.
            output::info(&format!("Packages: {:?}", config.pkgs));
            output::info(&format!("Services to enable: {:?}", config.services.enable));
            output::info(&format!(
                "Services to disable: {:?}",
                config.services.disable
            ));
        }
        Err(err) => {
            output::end(false);
            output::error(&format!("Failed to load garden: {err}"));
        }
    }
}
