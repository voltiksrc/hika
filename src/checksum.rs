use sha2::{Digest, Sha256};
use std::{error::Error, fs::File, io::Read, path::Path};

pub fn calculate_sha256(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        // Read up to 8192 bytes
        let bytes = file.read(&mut buffer)?;

        if bytes == 0 {
            break;
        }

        hash.update(&buffer[0..bytes]);
    }

    let result = hash.finalize();
    let sha256: String = result.iter().map(|byte| format!("{byte:02x}")).collect();

    Ok(sha256)
}

pub fn verify_sha256(path: &Path, expected: &str) -> Result<(), Box<dyn Error>> {
    let hash = calculate_sha256(path)?;

    if !hash.eq_ignore_ascii_case(expected) {
        return Err(format!(
            "SHA256 checksum mismatch\n    Expected: {expected}\n    Received: {hash}"
        )
        .into());
    }

    Ok(())
}
