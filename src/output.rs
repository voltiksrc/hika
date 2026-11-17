// SPDX-License-Identifier: GPL-3.0-only

use std::io::{self, Write};

// Shitty help menu
pub fn help() {
    println!("HIKA - binary based system manager");
    println!("Usage: hika <command>\n");
    println!("Options:");

    print_option("-Acquire <PACKAGE>", "Install a package");
    print_option("-Reacquire <PACKAGE>", "Reinstall a package");
    print_option("-Transcend", "Update system packages");
    print_option("--help", "Show this menu");
}

// TMP progress bar
pub fn progress(percent: usize) {
    let width = 20;
    let filled = percent.min(100) * width / 100;

    let dots = "•".repeat(filled);
    let empty = " ".repeat(width - filled);

    print!("\r  [\x1b[34m{dots}\x1b[0m{empty}] {percent}%");

    io::stdout().flush().unwrap();
}

fn print_option(command: &str, description: &str) {
    println!("  {:<24} {}", command, description);
}

// Output messages for normal, warnings, errors, and success.
pub fn info(message: &str) {
    println!("\x1b[34m[HIKA]:\x1b[0m {message}");
}

pub fn warn(message: &str) {
    eprintln!("\x1b[33m[HIKA]:\x1b[0m {message}");
}

pub fn error(message: &str) {
    eprintln!("\x1b[31m[HIKA]:\x1b[0m {message}");
}

pub fn success(message: &str) {
    println!("\x1b[32m[HIKA]:\x1b[0m {message}");
}

// Gentoo inspired messages
pub fn begin(message: &str) {
    print!("\x1b[34m[HIKA]:\x1b[0m {message}...");
    io::stdout().flush().unwrap();
}

pub fn end(success: bool) {
    if success {
        println!(" \x1b[32m[ ok ]\x1b[0m");
    } else {
        println!(" \x1b[31m[ !! ]\x1b[0m");
    }
}
