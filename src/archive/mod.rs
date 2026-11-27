mod detect;
mod extract;

use detect::{ArchiveFormat, detect_format};
use std::{error::Error, path::Path};

pub fn extract(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    match detect_format(source)? {
        ArchiveFormat::Tar => extract::extract_tar(source, destination),
        ArchiveFormat::TarGz => extract::extract_tar_gz(source, destination),
        ArchiveFormat::TarXz => extract::extract_tar_xz(source, destination),
        ArchiveFormat::TarBz2 => extract::extract_tar_bz2(source, destination),
        ArchiveFormat::TarZst => extract::extract_tar_zst(source, destination),
        ArchiveFormat::Deb => extract::extract_deb(source, destination),
    }
}
