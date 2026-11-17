use std::{fs, io};

pub fn hostname() -> io::Result<String> {
    Ok(fs::read_to_string("/etc/hostname")?.trim().to_string())
}

pub fn set_hostname(hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write("/etc/hostname", format!("{hostname}\n"))?;
    nix::unistd::sethostname(hostname)?;

    Ok(())
}
