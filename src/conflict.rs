// SPDX-License-Identifier: GPL-3.0-only

use std::{error::Error, fs, io::ErrorKind, path::Path};

pub fn check_conflicts(staging: &Path, root: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(staging)? {
        let entry = entry?;
        let source = entry.path();

        let relative = source.strip_prefix(staging)?;
        let destination = root.join(relative);

        let source_metadata = fs::symlink_metadata(&source)?;

        match fs::symlink_metadata(&destination) {
            Ok(metadata) => {
                if !(source_metadata.is_dir() && metadata.is_dir()) {
                    return Err(format!("file conflict: {}", destination.display()).into());
                }
            }

            Err(err) if err.kind() == ErrorKind::NotFound => {}

            Err(err) => return Err(err.into()),
        }

        if source_metadata.is_dir() {
            check_conflicts(&source, &destination)?;
        }
    }

    Ok(())
}
