use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use std::{error::Error, fs, fs::File, io::Read, path::Path};
use tar::Archive;
use xz2::read::XzDecoder;
use zstd::stream::read::Decoder;

// Helper function which creates the archive reader, the extraction directory, and extract the archive,
// so we don't repeat the same code throughout the entire file.
fn unpack<R: Read>(reader: R, destination: &Path) -> Result<(), Box<dyn Error>> {
    let mut archive = Archive::new(reader);

    fs::create_dir_all(destination)?;
    archive.unpack(destination)?;

    Ok(())
}

pub fn extract_tar(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(source)?;

    unpack(file, destination)
}

pub fn extract_tar_gz(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(source)?;
    // Create a new decoder which parses the gzip header
    let gz = GzDecoder::new(file);

    unpack(gz, destination)
}

pub fn extract_tar_xz(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(source)?;
    let xz = XzDecoder::new(file);

    unpack(xz, destination)
}

pub fn extract_tar_bz2(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(source)?;
    // Create a new decoder which parses the bzip header
    let bz2 = BzDecoder::new(file);

    unpack(bz2, destination)
}

pub fn extract_tar_zst(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(source)?;
    let zstd = Decoder::new(file)?;

    unpack(zstd, destination)
}

pub fn extract_deb(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(source)?;
    let mut archive = ar::Archive::new(file);

    // iterate through the embedded files in the .deb archive
    while let Some(entry) = archive.next_entry() {
        let entry = entry?;
        let name = std::str::from_utf8(entry.header().identifier())?;

        match name {
            "data.tar" => return unpack(entry, destination),
            "data.tar.xz" => {
                let xz = XzDecoder::new(entry);
                return unpack(xz, destination);
            }
            "data.tar.gz" => {
                let gz = GzDecoder::new(entry);
                return unpack(gz, destination);
            }
            "data.tar.bz2" => {
                let bz2 = BzDecoder::new(entry);
                return unpack(bz2, destination);
            }
            "data.tar.zst" => {
                let zstd = Decoder::new(entry)?;
                return unpack(zstd, destination);
            }
            _ => continue,
        }
    }

    Err("No supported data archive found in Debian package archive".into())
}
