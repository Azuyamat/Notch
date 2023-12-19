use serde::Deserialize;

#[derive(Deserialize)]
pub struct JarDetails {
    pub name: String,
    pub download_url: String,
    pub builds_url: String,
    pub versions_url: String,
}

pub struct Jar {
    pub name: String,
    pub version: Option<String>,
    pub build: Option<String>,
}