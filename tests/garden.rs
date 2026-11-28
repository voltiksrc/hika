// SPDX-License-Identifier: GPL-3.0-only

use hika::garden::load;
use std::fs;
use tempfile::tempdir;

#[test]
fn valid_config() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("system.lua");
    fs::write(&path, r#"system {
        hostname = "subjectx", timezone = "UTC", locale = "en_US.UTF-8", keymap = "us",
        pkgs = { "hello", "glibc" },
        services = { enable = { "sshd" }, disable = { "example" } },
    }"#).unwrap();
    let config = load(&path).unwrap();
    assert_eq!(config.hostname, "subjectx");
    assert_eq!(config.timezone, "UTC");
    assert_eq!(config.locale, "en_US.UTF-8");
    assert_eq!(config.keymap, "us");
    assert_eq!(config.pkgs, vec!["hello", "glibc"]);
    assert_eq!(config.services.enable, vec!["sshd"]);
    assert_eq!(config.services.disable, vec!["example"]);
}

#[test]
fn invalid_config() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("system.lua");
    for source in ["system {", "system {}", "local x = 1"] {
        fs::write(&path, source).unwrap();
        assert!(load(&path).is_err());
    }
}
