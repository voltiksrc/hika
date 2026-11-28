// SPDX-License-Identifier: GPL-3.0-only

use hika::{downloader::download, recipe::Artifact};
use std::fs;
use tempfile::tempdir;

fn artifact(urls: Vec<String>) -> Artifact {
    Artifact { urls, sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".into() }
}

#[test]
fn valid_cache_without_network() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("archive");
    fs::write(&path, b"abc").unwrap();
    download(&artifact(vec![]), &path).unwrap();
    assert_eq!(fs::read(path).unwrap(), b"abc");
}

#[test]
fn corrupted_cache_removed() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("archive");
    fs::write(&path, b"bad").unwrap();
    assert!(download(&artifact(vec![]), &path).is_err());
    assert!(!path.exists());
}

#[test]
fn existing_temp_preserved() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("archive");
    let temp = dir.path().join("archive.tmp");
    fs::write(&temp, b"keep").unwrap();
    assert!(download(&artifact(vec![]), &path).is_err());
    assert_eq!(fs::read(temp).unwrap(), b"keep");
}

#[test]
fn invalid_mirrors_reported_without_network() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("archive");
    let err = download(&artifact(vec!["not a url".into(), "http://example.invalid/archive".into()]), &path).unwrap_err().to_string();
    assert!(err.contains("Invalid URL"));
    assert!(err.contains("Only HTTPS"));
    assert!(!path.exists());
    assert!(!dir.path().join("archive.tmp").exists());
}
