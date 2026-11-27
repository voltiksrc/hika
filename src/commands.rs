// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;

use crate::{
    acquire, garden,
    garden::reconcile::{self, Change},
    output,
};

pub fn acquire(packages: Vec<String>) {
    for package in packages {
        if let Err(err) = crate::acquire::acquire_package(&package) {
            output::error(&format!("Failed to acquire {package}: {err}"));
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
