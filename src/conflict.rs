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
                if !source_metadata.is_dir() && metadata.is_dir() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn missing_destination() {
        let staging = tempdir().unwrap();
        let root = tempdir().unwrap();

        fs::write(staging.path().join("hello"), "hello").unwrap();

        assert!(check_conflicts(staging.path(), root.path()).is_ok());
    }

    #[test]
    fn existing_file() {
        let staging = tempdir().unwrap();
        let root = tempdir().unwrap();

        fs::write(staging.path().join("hello"), "new").unwrap();
        fs::write(root.path().join("hello"), "old").unwrap();

        assert!(check_conflicts(staging.path(), root.path()).is_err());
    }

    #[test]
    fn shared_directory() {
        let staging = tempdir().unwrap();
        let root = tempdir().unwrap();

        fs::create_dir(staging.path().join("usr")).unwrap();
        fs::create_dir(root.path().join("usr")).unwrap();

        assert!(check_conflicts(staging.path(), root.path()).is_ok());
    }

    #[test]
    fn nested_conflict() {
        let staging = tempdir().unwrap();
        let root = tempdir().unwrap();

        for dir in [&staging, &root] {
            fs::create_dir_all(dir.path().join("usr/bin")).unwrap();
            fs::write(dir.path().join("usr/bin/hello"), "hello").unwrap();
        }

        assert!(check_conflicts(staging.path(), root.path()).is_err());
    }

    #[test]
    fn directory_file_conflict() {
        let staging = tempdir().unwrap();
        let root = tempdir().unwrap();

        fs::create_dir(staging.path().join("usr")).unwrap();
        fs::write(root.path().join("usr"), "hello").unwrap();

        assert!(check_conflicts(staging.path(), root.path()).is_err());
    }

    #[test]
    fn dangling_symlink_conflict() {
        use std::os::unix::fs::symlink;

        let staging = tempdir().unwrap();
        let root = tempdir().unwrap();

        fs::write(staging.path().join("hello"), "hello").unwrap();
        symlink("missing", root.path().join("hello")).unwrap();

        assert!(check_conflicts(staging.path(), root.path()).is_err());
    }
}
