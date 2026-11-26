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
    cli::run();
}
