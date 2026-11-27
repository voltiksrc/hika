// SPDX-License-Identifier: GPL-3.0-only

mod acquire;
mod archive;
mod checksum;
mod cli;
mod commands;
mod downloader;
mod garden;
mod output;
mod paths;
mod recipe;

fn main() {
    cli::run();
}
