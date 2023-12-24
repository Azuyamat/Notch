use crate::error::Error;
use crate::servers::server::Server;
use std::fs;
use std::io::Write;
use crate::hangar::fetch_plugin_info;

pub struct Manipulator<'a> {
    server: &'a Server,
}

impl<'a> Manipulator<'a> {
    pub fn new(server: &'a Server) -> Self {
        Self { server }
    }

    /// Get the server's plugins' names
    pub fn plugins(&self) -> Result<Vec<String>, Error> {
        let plugins_dir = self.server.location.join("plugins");
        let plugins = Vec::new();
        if !plugins_dir.exists() {
            return Ok(plugins);
        }
        let plugins = fs::read_dir(plugins_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().unwrap_or_default() == "jar")
            .map(|entry| entry.file_name())
            .map(|file_name| file_name.to_str().unwrap_or_default().trim_end_matches(".jar").to_string())
            .collect();
        Ok(plugins)
    }

    /// Remove a plugin
    pub fn remove_plugin(&self, plugin: &str) -> Result<(), Error> {
        let plugin_path = self.server.location
            .join("plugins")
            .join(format!("{}.jar", plugin));
        if !plugin_path.exists() {
            return Err(Error::PluginNotFound);
        }
        fs::remove_file(plugin_path)?;
        Ok(())
    }

    /// Download a plugin from Hangar (https://hangar.papermc.io/api/v1/projects/AUTHOR/SLUG/versions)
    pub fn download_plugin(&self, author: &str, slug: &str) -> Result<(), Error> {
        let plugin = fetch_plugin_info(author, slug)?;
        let downloads = &plugin.first().ok_or(Error::PluginNotFound)?.downloads;
        // Set download_url to either velocity, waterfall, or paper depending on which matches the server's jar name
        let jar_name = self.server.jar.name.to_lowercase();
        let download_url = match jar_name.as_str() {
            "velocity" => downloads.velocity.as_ref(),
            "waterfall" => downloads.waterfall.as_ref(),
            "paper" => downloads.paper.as_ref(),
            _ => None
        };
        if download_url.is_none() {
            return Err(Error::PluginNotFound);
        }
        // Set download_url to either externalUrl or downloadUrl depending on which is not None
        let download_url = download_url.unwrap();
        let download_url: &String = download_url.download_url.as_ref().ok_or(Error::DownloadUrlNotFound)?;
        let response = reqwest::blocking::get(download_url)?;
        // Make sure response is of type Jar
        if let Some(content_type) = response.headers().get("content-type") {
            if content_type != "application/java-archive" {
                return Err(Error::DownloadUrlNotJar);
            }
        }
        let mut file = fs::File::create(self.server.location.join("plugins").join(format!("{}.jar", slug)))?;
        let bytes = response.bytes()?;
        let chunks = bytes.chunks(1024);
        for chunk in chunks {
            file.write_all(chunk)?;
        }
        Ok(())
    }
}