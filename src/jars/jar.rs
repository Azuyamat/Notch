use crate::error::Error;
use crate::jars::request::{JarBuildsPaper, JarBuildsPurpur, JarVersions};
use serde::{Deserialize, Serialize};

/// JarDetails is a struct that contains the details of a jar. It is used to get the latest version and build of a jar.
#[derive(Deserialize)]
pub struct JarDetails {
    pub name: String,
    pub download_url: String,
    pub builds_url: String,
    pub versions_url: String,
}

impl JarDetails {
    pub fn get_versions(&self) -> Result<Vec<String>, Error> {
        let mut versions: JarVersions = reqwest::blocking::get(&self.versions_url)?.json()?;
        versions.versions.reverse();
        Ok(versions.versions)
    }
    pub fn get_latest_version(&self) -> Result<String, Error> {
        let versions = self.get_versions()?;
        Ok(versions.first().ok_or(Error::NoVersionsFound)?.clone())
    }
    pub fn get_builds(&self, version: String) -> Result<Vec<u32>, Error> {
        let url = &self.builds_url.replace("{version}", &version);
        let builds: Vec<u32> = match &self.name.to_lowercase().as_str() {
            &"purpur" => {
                let response: JarBuildsPurpur = reqwest::blocking::get(url)?.json()?;
                let builds = response
                    .builds
                    .all
                    .iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                builds
            }
            _ => {
                let response: JarBuildsPaper = reqwest::blocking::get(url)?.json()?;
                response.builds
            }
        };
        Ok(builds)
    }
    pub fn get_latest_build(&self, version: String) -> Result<u32, Error> {
        let builds = self.get_builds(version)?;
        builds.first().copied().ok_or(Error::NoBuildsFound)
    }
    pub fn get_latest(&self) -> Result<Jar, Error> {
        let version = self.get_latest_version()?;
        let build = self.get_latest_build(version.clone())?;
        Ok(Jar {
            name: self.name.clone(),
            version: Some(version),
            build: Some(build),
        })
    }
}

/// Jar is a struct that contains the name, version, and build of a jar.
#[derive(Deserialize, Serialize)]
pub struct Jar {
    pub name: String,
    pub version: Option<String>,
    pub build: Option<u32>,
}
