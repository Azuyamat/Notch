use crate::error::Error;
use crate::jars::manager::JarManager;
use crate::servers::server::Server;
use std::io::Write;

/// Downloads the server jar file.
pub struct Downloader<'a> {
    pub server: &'a Server,
}

impl<'a> Downloader<'a> {
    pub fn new(server: &'a Server) -> Self {
        Self { server }
    }

    /// Downloads the server jar file.
    pub fn download(&self) -> Result<(), Error> {
        let jars = JarManager::load()?;
        let jar = &self.server.jar;
        let version = jar.version.clone().unwrap_or("latest".to_string());
        let build = jar.build.unwrap_or(0);
        let jar_path = &self
            .server
            .location
            .join(format!("{}-{}.jar", &jar.name, &version));

        let jar_details = jars
            .jars
            .iter()
            .find(|j| j.name == jar.name)
            .ok_or(Error::JarNotFound)?;
        let download_url = &jar_details
            .download_url
            .replace("{version}", &version)
            .replace("{build}", &build.to_string());

        let response = reqwest::blocking::get(download_url)?;
        let mut file = std::fs::File::create(jar_path)?;

        let bytes = response.bytes()?;
        let chunks = bytes.chunks(1024);
        for chunk in chunks {
            file.write_all(chunk)?;
        }

        Ok(())
    }
}
