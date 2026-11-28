// SPDX-License-Identifier: GPL-3.0-only

use hika::merge::merge;
use std::{fs, os::unix::fs::{symlink, PermissionsExt}, path::PathBuf};
use tempfile::tempdir;

#[test]
fn nested_files_and_manifest() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::create_dir_all(stage.path().join("usr/bin")).unwrap();
    fs::create_dir(root.path().join("usr")).unwrap();
    fs::write(stage.path().join("usr/bin/hello"), b"hello").unwrap();
    let record = merge(stage.path(), root.path()).unwrap();
    assert_eq!(fs::read(root.path().join("usr/bin/hello")).unwrap(), b"hello");
    assert_eq!(record.files, vec![PathBuf::from("usr/bin/hello")]);
    assert_eq!(record.directories, vec![PathBuf::from("usr/bin")]);
}

#[test]
fn permissions() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    let source = stage.path().join("hello");
    fs::write(&source, b"hello").unwrap();
    fs::set_permissions(&source, fs::Permissions::from_mode(0o6755)).unwrap();
    merge(stage.path(), root.path()).unwrap();
    assert_eq!(fs::metadata(root.path().join("hello")).unwrap().permissions().mode() & 0o7777, 0o755);
}

#[test]
fn no_overwrite() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::write(stage.path().join("file"), "new").unwrap();
    fs::write(root.path().join("file"), "old").unwrap();
    assert!(merge(stage.path(), root.path()).is_err());
    assert_eq!(fs::read_to_string(root.path().join("file")).unwrap(), "old");
}

#[test]
fn relative_symlink() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::create_dir_all(stage.path().join("usr/bin")).unwrap();
    symlink("../lib/hello", stage.path().join("usr/bin/link")).unwrap();
    let record = merge(stage.path(), root.path()).unwrap();
    assert_eq!(fs::read_link(root.path().join("usr/bin/link")).unwrap(), PathBuf::from("../lib/hello"));
    assert_eq!(record.files, vec![PathBuf::from("usr/bin/link")]);
}

#[test]
fn absolute_symlink_rejected() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    symlink("/example", stage.path().join("link")).unwrap();
    assert!(merge(stage.path(), root.path()).is_err());
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[test]
fn escaping_symlink_rolls_back_directories() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    fs::create_dir_all(stage.path().join("usr/bin")).unwrap();
    symlink("../../../outside", stage.path().join("usr/bin/link")).unwrap();
    fs::create_dir(root.path().join("usr")).unwrap();
    fs::write(root.path().join("keep"), "keep").unwrap();
    assert!(merge(stage.path(), root.path()).is_err());
    assert!(root.path().join("usr").is_dir());
    assert!(!root.path().join("usr/bin").exists());
    assert_eq!(fs::read_to_string(root.path().join("keep")).unwrap(), "keep");
}

#[test]
fn destination_directory_symlink_rejected() {
    let stage = tempdir().unwrap();
    let root = tempdir().unwrap();
    let outside = tempdir().unwrap();
    fs::create_dir(stage.path().join("usr")).unwrap();
    fs::write(stage.path().join("usr/file"), "new").unwrap();
    symlink(outside.path(), root.path().join("usr")).unwrap();
    assert!(merge(stage.path(), root.path()).is_err());
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
}
