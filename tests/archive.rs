// SPDX-License-Identifier: GPL-3.0-only

use hika::archive::extract;
use std::{fs, io::Write};
use tempfile::tempdir;

fn tar_bytes() -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    let mut header = tar::Header::new_ustar();
    header.set_size(5);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append_data(&mut header, "usr/share/hello", &b"hello"[..]).unwrap();
    builder.into_inner().unwrap()
}

fn check(bytes: &[u8]) {
    let dir = tempdir().unwrap();
    let source = dir.path().join("archive");
    let dest = dir.path().join("out");
    fs::write(&source, bytes).unwrap();
    extract(&source, &dest).unwrap();
    assert_eq!(fs::read(dest.join("usr/share/hello")).unwrap(), b"hello");
}

#[test]
fn tar() { check(&tar_bytes()); }

#[test]
fn gzip() {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&tar_bytes()).unwrap();
    check(&encoder.finish().unwrap());
}

#[test]
fn bzip2() {
    let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
    encoder.write_all(&tar_bytes()).unwrap();
    check(&encoder.finish().unwrap());
}

#[test]
fn xz() {
    let mut encoder = xz2::write::XzEncoder::new(Vec::new(), 6);
    encoder.write_all(&tar_bytes()).unwrap();
    check(&encoder.finish().unwrap());
}

#[test]
fn zstd() { check(&zstd::stream::encode_all(&tar_bytes()[..], 1).unwrap()); }

#[test]
fn deb() {
    let bytes = tar_bytes();
    let mut builder = ar::Builder::new(Vec::new());
    let header = ar::Header::new(b"data.tar".to_vec(), bytes.len() as u64);
    builder.append(&header, &bytes[..]).unwrap();
    check(&builder.into_inner().unwrap());
}

#[test]
fn unsupported() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("archive");
    fs::write(&source, b"not an archive").unwrap();
    assert!(extract(&source, &dir.path().join("out")).is_err());
}

#[test]
fn truncated_gzip() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("archive");
    fs::write(&source, [0x1f, 0x8b, 0x08]).unwrap();
    assert!(extract(&source, &dir.path().join("out")).is_err());
}
