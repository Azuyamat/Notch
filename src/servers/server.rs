use std::fs;
use std::path::PathBuf;
use crate::jars::jar::Jar;
use crate::servers::settings::ServerSettings;
use crate::error::Error;

/// Represents a server
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

    pub fn delete(&self) -> Result<(), Error> {
        fs::remove_dir_all(&self.location)?;
        Ok(())
    }
}

