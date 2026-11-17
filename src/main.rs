// SPDX-License-Identifier: GPL-3.0-only

mod cli;
mod commands;
mod downloader;
mod output;
mod recipe;

fn main() {
    cli::run();
}
