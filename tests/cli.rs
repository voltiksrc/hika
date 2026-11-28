// SPDX-License-Identifier: GPL-3.0-only

use std::process::Command;
use tempfile::tempdir;

#[test]
fn help() {
    for args in [&[][..], &["--help"][..], &["-h"][..]] {
        let output = Command::new(env!("CARGO_BIN_EXE_hika")).args(args).output().unwrap();
        assert!(output.status.success());
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(text.contains("-Acquire"));
        assert!(text.contains("-Reacquire"));
        assert!(text.contains("-Transcend"));
        assert!(text.contains("-Reconcile"));
    }
}

#[test]
fn version() {
    let output = Command::new(env!("CARGO_BIN_EXE_hika")).arg("--version").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn unknown_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_hika")).arg("unknown-command").output().unwrap();
    assert!(!output.status.success());
}

#[test]
fn acquire_missing_recipe_reports_error() {
    let dir = tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_hika")).current_dir(dir.path()).args(["-Acquire", "missing"]).output().unwrap();
    let text = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    assert!(text.contains("Failed to acquire missing"));
}
