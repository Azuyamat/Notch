use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::jars::jar::Jar;
use crate::servers::settings::ServerSettings;
use crate::error::Error;

/// Represents a server
#[derive(Deserialize, Serialize)]
pub struct Server {
    pub name: String,
    pub jar: Jar,
    pub location: PathBuf,
    pub settings: ServerSettings,
}

impl Server {
    pub fn new(name: String, jar: Jar, location: PathBuf) -> Result<Self, Error> {
        if !location.exists() {
            return Err(Error::PathDoesNotExist(location));
        }
        if location.is_relative() {
            return Err(Error::PathIsRelative(location));
        }
        Ok(Self {
            name,
            jar,
            location,
            settings: ServerSettings::default(),
        })
    }

    pub fn get_jar_path(&self) -> Result<PathBuf, Error> {
        let directory = &self.location;
        let jars = std::fs::read_dir(directory)?;
        let jar = jars
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                let path = entry.path();
                let extension = path.extension();
                extension.is_some() && extension.unwrap() == "jar"
            })
            .ok_or(Error::NoJarFound)?;
        Ok(jar.path())
    }

    pub fn accept_eula(&self) -> Result<(), Error> {
        let eula_path = self.location.join("eula.txt");
        fs::write(eula_path, "eula=true")?;
        Ok(())
    }

    /// Write to notch.toml
    pub fn save(&self) -> Result<(), Error> {
        let config_path = self.location.join("notch.toml");
        let config = toml::to_string(&self)?;
        fs::write(config_path, config)?;
        Ok(())
    }

    pub fn delete(&self) -> Result<(), Error> {
        fs::remove_dir_all(&self.location)?;
        Ok(())
    }

    /// Read from notch.toml
    pub fn from_path(path: PathBuf) -> Result<Self, Error> {
        let config_path = path.join("notch.toml");
        if !config_path.exists() {
            return Err(Error::ServerConfigNotFound(path));
        }
        let config = fs::read_to_string(config_path)?;
        let server: Server = toml::from_str(&config)?;
        Ok(server)
    }
}

