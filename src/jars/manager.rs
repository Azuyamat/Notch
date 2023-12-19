use crate::jars::jar::JarDetails;
use serde::Deserialize;
use crate::error::Error;

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
}