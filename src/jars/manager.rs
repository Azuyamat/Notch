use serde::Deserialize;
use crate::jars::jar::JarDetails;


const JARS_TOML: &str = include_str!("../../jars.toml");

pub fn load() -> Result<Jars, toml::de::Error> {
    toml::from_str(JARS_TOML)
}

#[derive(Deserialize)]
pub struct Jars {
    pub jars: Vec<JarDetails>,
}