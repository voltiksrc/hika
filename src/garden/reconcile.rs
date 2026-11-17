use crate::garden::config::SystemConfig;

pub enum Change {
    SetHostname { old: String, new: String },
}

pub fn compare_hostname(
    config: &SystemConfig,
) -> Result<Option<Change>, Box<dyn std::error::Error>> {
    let current_hostname = std::fs::read_to_string("/etc/hostname")?;
    let current = current_hostname.trim();

    if config.hostname != current {
        return Ok(Some(Change::SetHostname {
            old: current.to_string(),
            new: config.hostname.clone(),
        }));
    }

    Ok(None)
}
