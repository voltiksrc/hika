use mlua::{Lua, Table};
use std::{cell::RefCell, error::Error, fs, path::Path, rc::Rc};

pub struct SystemConfig {
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub keymap: String,
    pub pkgs: Vec<String>,
    pub services: Services,
}

pub struct Services {
    pub enable: Vec<String>,
    pub disable: Vec<String>,
}

pub fn load(path: &Path) -> Result<SystemConfig, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let lua = Lua::new();

    // We let system() write the full system config, and then read it
    // back after all sections are filled out.
    let parsed_config = Rc::new(RefCell::new(None::<SystemConfig>));
    let parsed_config_ref = Rc::clone(&parsed_config);

    // Lua calls this when it encounters system { ... }
    let system = lua.create_function(move |_, table: Table| {
        let hostname: String = table.get("hostname")?;
        let timezone: String = table.get("timezone")?;
        let locale: String = table.get("locale")?;
        let keymap: String = table.get("keymap")?;

        // Read the packages from the lua table.
        let plants: Table = table.get("pkgs")?;
        let mut pkgs = Vec::new();

        for plant in plants.sequence_values::<String>() {
            pkgs.push(plant?);
        }

        // Services are separated into enabled and disabled lists.
        let services: Table = table.get("services")?;
        let enabled: Table = services.get("enable")?;
        let disabled: Table = services.get("disable")?;
        let mut enabled_services = Vec::new();
        let mut disabled_services = Vec::new();

        for service in enabled.sequence_values::<String>() {
            enabled_services.push(service?);
        }

        for service in disabled.sequence_values::<String>() {
            disabled_services.push(service?);
        }

        let services = Services {
            enable: enabled_services,
            disable: disabled_services,
        };

        let systemconfig = SystemConfig {
            hostname,
            timezone,
            locale,
            keymap,
            pkgs,
            services,
        };

        *parsed_config_ref.borrow_mut() = Some(systemconfig);

        Ok(())
    })?;

    // Register the function before executing the configuration.
    lua.globals().set("system", system)?;
    lua.load(&source).exec()?;

    parsed_config
        .borrow_mut()
        .take()
        .ok_or_else(|| "configuration did not call system { ... }".into())
}

pub fn check_hostname(config: &SystemConfig) -> Result<(), Box<dyn Error>> {
    let current = fs::read_to_string("/etc/hostname")?;
    let current = current.trim();

    if current != config.hostname {
        println!("  ~ Hostname: {current} → {}", config.hostname);
    }

    Ok(())
}
