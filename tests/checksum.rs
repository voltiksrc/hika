// SPDX-License-Identifier: GPL-3.0-only

use hika::checksum::{calculate_sha256, verify_sha256};
use std::fs;
use tempfile::tempdir;

const ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

#[test]
fn known_digest() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("file");
    fs::write(&path, b"abc").unwrap();
    assert_eq!(calculate_sha256(&path).unwrap(), ABC);
    verify_sha256(&path, ABC).unwrap();
    verify_sha256(&path, &ABC.to_uppercase()).unwrap();
}

#[test]
fn empty_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("file");
    fs::write(&path, b"").unwrap();
    assert_eq!(calculate_sha256(&path).unwrap(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[test]
fn wrong_checksum() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("file");
    fs::write(&path, b"wrong").unwrap();
    let err = verify_sha256(&path, ABC).unwrap_err();
    assert!(err.to_string().contains("checksum mismatch"));
}

#[test]
fn missing_file() {
    let dir = tempdir().unwrap();
    assert!(calculate_sha256(&dir.path().join("missing")).is_err());
}
