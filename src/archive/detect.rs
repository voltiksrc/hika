use std::{error::Error, fs::File, io::Read, path::Path};

#[derive(Debug, PartialEq, Eq)]
pub enum ArchiveFormat {
    Tar,
    TarGz,
    TarXz,
    TarBz2,
    TarZst,
    Deb,
}

pub fn detect_format(source: &Path) -> Result<ArchiveFormat, Box<dyn Error>> {
    let file = File::open(source)?;
    let mut header = Vec::new();

    // Read up to 512 bytes, stopping at EOF if the file is smaller than that
    file.take(512).read_to_end(&mut header)?;

    if header.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]) {
        return Ok(ArchiveFormat::TarXz);
    }

    if header.starts_with(&[0x1F, 0x8B]) {
        return Ok(ArchiveFormat::TarGz);
    }

    if header.starts_with(&[0x28, 0xB5, 0x2F, 0xFD]) {
        return Ok(ArchiveFormat::TarZst);
    }

    if header.starts_with(&[0x42, 0x5A, 0x68]) {
        return Ok(ArchiveFormat::TarBz2);
    }

    if header.starts_with(b"!<arch>\n") {
        return Ok(ArchiveFormat::Deb);
    }

    // Tar's signature isnt at the beginning so we use get instead, so we prevent a panic
    // if the file is too small
    if header.get(257..262) == Some(b"ustar") {
        return Ok(ArchiveFormat::Tar);
    }

    Err("Unsupported archive format".into())
}
