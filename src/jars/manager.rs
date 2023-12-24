use crate::error::Error;
use crate::jars::jar::JarDetails;
use serde::Deserialize;

const JARS_TOML: &str = include_str!("../../jars.toml");

/// JarManager is a struct that contains a list of jars. It is used to get the details of a jar.
#[derive(Deserialize)]
pub struct JarManager {
    pub jars: Vec<JarDetails>,
}

impl JarManager {
    pub fn load() -> Result<Self, Error> {
        Ok(toml::from_str(JARS_TOML)?)
    }

    pub fn get_jar(&self, name: String) -> Result<&JarDetails, Error> {
        let jar = self
            .jars
            .iter()
            .find(|jar| jar.name.eq_ignore_ascii_case(&name));
        jar.ok_or(Error::JarNotFound)
    }
}
