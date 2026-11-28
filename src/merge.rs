// SPDX-License-Identifier: GPL-3.0-only

use std::{
    error::Error,
    fs, io,
    os::unix::fs::PermissionsExt,
    path::{Component, Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct Merge {
    pub files: Vec<PathBuf>,
    pub directories: Vec<PathBuf>,
}

fn rollback(root: &Path, deployment: &Merge) -> Result<(), Box<dyn Error>> {
    // Remove installed files first
    for relative in deployment.files.iter().rev() {
        let destination = root.join(relative);

        fs::remove_file(destination)?;
    }

    // Remove newly created directories from deepest to shallowest
    for relative in deployment.directories.iter().rev() {
        let destination = root.join(relative);

        fs::remove_dir(destination)?;
    }

    Ok(())
}

pub fn merge(staging: &Path, root: &Path) -> Result<Merge, Box<dyn Error>> {
    let mut deployment = Merge::default();

    // merge_entries takes 4 parameters, the first staging parameter is the original staging
    // directory which never changes. The second staging parameter is the directory we're currently traversing.
    // The third parameter is of course, the root of the filesystem which is our installation destination.
    // The 4th parameter is the recorded files and directories shared across recursive calls which we will use for rollbacks.
    match merge_entries(staging, staging, root, &mut deployment) {
        Ok(()) => Ok(deployment),

        Err(err) => {
            // Undo any changes made before the merge failed
            if let Err(rollback_err) = rollback(root, &deployment) {
                return Err(
                    format!("merge failed: {}; rollback failed: {}", err, rollback_err).into(),
                );
            }

            Err(err)
        }
    }
}

fn merge_entries(
    staging: &Path,
    current: &Path,
    root: &Path,
    deployment: &mut Merge,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let source = entry.path();

        let relative = source.strip_prefix(staging)?;
        let destination = root.join(relative);
        let metadata = fs::symlink_metadata(&source)?;

        if metadata.is_dir() {
            create_directory(&destination, relative, deployment)?;
            merge_entries(staging, &source, root, deployment)?;
        } else if metadata.is_file() {
            copy_regular_file(&source, &destination, deployment, relative)?;
        } else if metadata.is_symlink() {
            copy_symlink(&source, &destination, relative, deployment)?;
        } else {
            return Err(format!("unsupported file type: {}", relative.display()).into());
        }
    }

    Ok(())
}

fn validate_link(relative: &Path, target: &Path) -> Result<(), Box<dyn Error>> {
    // Reject empty targets and links pointing to absolute paths
    if target.as_os_str().is_empty() || target.is_absolute() {
        return Err(format!("invalid symlink target: {}", target.display()).into());
    }

    // Relative symlink targets are resolved from the link's parent directory
    let parent = relative.parent().ok_or("missing parent")?;
    let joined = parent.join(target);

    // Track how far we are from the installation root
    let mut depth = 0;

    for component in joined.components() {
        match component {
            Component::Normal(_) => {
                depth += 1;
            }

            Component::ParentDir => {
                // Allow ".." as long as it doesn't escape the root
                if depth == 0 {
                    return Err("following parent dir would escape root".into());
                }

                depth -= 1;
            }

            Component::CurDir => {}

            _ => return Err("invalid path component".into()),
        }
    }

    Ok(())
}

fn copy_regular_file(
    source: &Path,
    destination: &Path,
    deployment: &mut Merge,
    relative: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut input = fs::File::open(source)?;
    let metadata = input.metadata()?;

    if !metadata.is_file() {
        return Err("source is not a regular file".into());
    }

    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true) // hika should not overwrite an existing file
        .open(destination)?;

    deployment.files.push(relative.to_path_buf());

    io::copy(&mut input, &mut output)?;

    // Retain executable permissions otherwise binaries could be unusable
    let perms = metadata.permissions().mode() & 0o777;
    output.set_permissions(fs::Permissions::from_mode(perms))?;

    Ok(())
}

fn copy_symlink(
    source: &Path,
    destination: &Path,
    relative: &Path,
    deployment: &mut Merge,
) -> Result<(), Box<dyn Error>> {
    let target = fs::read_link(source)?;

    // Install and track the validated symlink
    validate_link(relative, &target)?;
    std::os::unix::fs::symlink(&target, destination)?;

    deployment.files.push(relative.to_path_buf());

    Ok(())
}

fn create_directory(
    destination: &Path,
    relative: &Path,
    deployment: &mut Merge,
) -> Result<(), Box<dyn Error>> {
    match fs::create_dir(destination) {
        Ok(()) => {
            deployment.directories.push(relative.to_path_buf());
        }

        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(destination)?;

            // Existing directories can be shared between packages
            if !metadata.is_dir() {
                return Err(format!("directory conflict: {}", destination.display()).into());
            }
        }

        Err(err) => return Err(err.into()),
    }

    Ok(())
}
