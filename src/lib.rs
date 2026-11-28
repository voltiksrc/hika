// SPDX-License-Identifier: GPL-3.0-only

pub mod acquire;
pub mod archive;
pub mod checksum;
pub mod conflict;
pub mod downloader;
pub mod garden;
pub mod merge;
pub mod recipe;

mod cli;
mod commands;
mod output;
mod paths;

pub fn run() {
    cli::run();
}
