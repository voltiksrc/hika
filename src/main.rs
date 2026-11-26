// SPDX-License-Identifier: GPL-3.0-only

mod checksum;
mod cli;
mod commands;
mod downloader;
mod extractor;
mod garden;
mod output;
mod paths;
mod recipe;

fn main() {
    let source = std::path::Path::new("cache/downloads/hello-2.12.3.deb");
    let destination = &paths::staging().join("hello-2.12.3");

    match extractor::extract_deb(source, destination) {
        Ok(()) => println!("successful!"),
        Err(err) => eprintln!("failed: {err:?}"),
    }

    //cli::run();
}
