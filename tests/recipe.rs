// SPDX-License-Identifier: GPL-3.0-only

use hika::recipe::load_recipe;
use std::fs;
use tempfile::tempdir;

const RECIPE: &str = r#"pkg {
    name = "hello", version = "2.12.3", description = "test package",
    artifacts = {
        { urls = { "https://example.invalid/a", "https://example.invalid/b" }, sha256 = "abc" },
        { urls = { "https://example.invalid/c" }, sha256 = "def" },
    },
    runtime_deps = { "glibc", "example" },
}"#;

#[test]
fn valid_recipe() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("package.lua");
    fs::write(&path, RECIPE).unwrap();
    let pkg = load_recipe(&path).unwrap();
    assert_eq!(pkg.name, "hello");
    assert_eq!(pkg.version, "2.12.3");
    assert_eq!(pkg.description, "test package");
    assert_eq!(pkg.artifacts.len(), 2);
    assert_eq!(pkg.artifacts[0].urls.len(), 2);
    assert_eq!(pkg.artifacts[1].sha256, "def");
    assert_eq!(pkg.runtime_deps, vec!["glibc", "example"]);
}

#[test]
fn optional_dependencies() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("package.lua");
    fs::write(&path, RECIPE.replace("runtime_deps = { \"glibc\", \"example\" },", "")).unwrap();
    assert!(load_recipe(&path).unwrap().runtime_deps.is_empty());
}

#[test]
fn invalid_recipes() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("package.lua");
    for source in ["pkg {", "pkg {}", "local x = 1", "pkg { name = {} }"] {
        fs::write(&path, source).unwrap();
        assert!(load_recipe(&path).is_err(), "accepted {source}");
    }
}

#[test]
fn missing_recipe() {
    let dir = tempdir().unwrap();
    assert!(load_recipe(&dir.path().join("missing.lua")).is_err());
}
