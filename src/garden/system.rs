use std::{fs, io};

pub fn hostname() -> io::Result<String> {
    Ok(fs::read_to_string("/etc/hostname")?.trim().to_string())
}
