use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to find path {0}")]
    PathDoesNotExist(PathBuf),
    #[error("Path {0} is relative")]
    PathIsRelative(PathBuf),
    #[error("Failed to find jar")]
    JarNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse toml file")]
    TomlError(#[from] toml::de::Error),
    #[error("Failed to serialize toml file")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to find builds")]
    NoBuildsFound,
    #[error("Failed to find versions")]
    NoVersionsFound,
    #[error("Failed to find stdout")]
    NoStdout,
    #[error("Failed to find jar")]
    NoJarFound,
    #[error("Failed to find server config")]
    ServerConfigNotFound(PathBuf),
}