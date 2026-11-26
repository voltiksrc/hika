use std::path::PathBuf;

pub fn cache() -> PathBuf {
    PathBuf::from("cache")
}

pub fn downloads() -> PathBuf {
    cache().join("downloads")
}

pub fn staging() -> PathBuf {
    cache().join("staging")
}

pub fn root() -> PathBuf {
    PathBuf::from("root")
}

pub fn recipe(package: &str) -> PathBuf {
    PathBuf::from("recipe").join(package).join("package.lua")
}
