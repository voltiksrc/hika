// SPDX-License-Identifier: GPL-3.0-only

use hika::conflict::check_conflicts;
use std::{fs, os::unix::fs::symlink};
use tempfile::tempdir;

#[test]
fn missing_destination() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::write(stage.path().join("file"), "new").unwrap();
    check_conflicts(stage.path(), root.path()).unwrap();
}

#[test]
fn shared_directory() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    for dir in [&stage, &root] { fs::create_dir(dir.path().join("usr")).unwrap(); }
    check_conflicts(stage.path(), root.path()).unwrap();
}

#[test]
fn existing_file() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::write(stage.path().join("file"), "new").unwrap();
    fs::write(root.path().join("file"), "old").unwrap();
    assert!(check_conflicts(stage.path(), root.path()).is_err());
}

#[test]
fn nested_conflict() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    for dir in [&stage, &root] {
        fs::create_dir_all(dir.path().join("usr/bin")).unwrap();
        fs::write(dir.path().join("usr/bin/hello"), "hello").unwrap();
    }
    assert!(check_conflicts(stage.path(), root.path()).is_err());
}

#[test]
fn file_against_directory() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::write(stage.path().join("entry"), "new").unwrap();
    fs::create_dir(root.path().join("entry")).unwrap();
    assert!(check_conflicts(stage.path(), root.path()).is_err());
}

#[test]
fn directory_against_file() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::create_dir(stage.path().join("entry")).unwrap();
    fs::write(root.path().join("entry"), "old").unwrap();
    assert!(check_conflicts(stage.path(), root.path()).is_err());
}

#[test]
fn dangling_symlink() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::write(stage.path().join("entry"), "new").unwrap();
    symlink("missing", root.path().join("entry")).unwrap();
    assert!(check_conflicts(stage.path(), root.path()).is_err());
}

#[test]
fn directory_symlink() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::create_dir(stage.path().join("entry")).unwrap();
    fs::create_dir(root.path().join("real")).unwrap();
    symlink("real", root.path().join("entry")).unwrap();
    assert!(check_conflicts(stage.path(), root.path()).is_err());
}
