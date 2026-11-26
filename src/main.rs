// SPDX-License-Identifier: GPL-3.0-only

mod checksum;
mod cli;
mod commands;
mod downloader;
mod garden;
mod output;
mod recipe;

fn main() {
    cli::run();
}
