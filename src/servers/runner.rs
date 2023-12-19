use std::process::{Child, Stdio};
use crate::error::Error;
use crate::servers::server::Server;

/// A struct that represents a server runner.
pub struct Runner<'a> {
    server: &'a Server,
    java_args: Vec<&'a str>,
    server_args: Vec<&'a str>,
    child: Option<Child>,
}

impl<'a> Runner<'a> {
    pub fn new(server: &'a Server, java_args: Vec<&'a str>, server_args: Vec<&'a str>) -> Self {
        Self {
            server,
            java_args,
            server_args,
            child: None,
        }
    }

    /// Starts the server.
    pub fn start(&mut self) -> Result<Child, Error> {
        let initial_memory = self.server.settings.initial_memory.prepend_flag("-Xms");
        let max_memory = self.server.settings.max_memory.prepend_flag("-Xmx");
        let default_args: Vec<String> = [initial_memory, max_memory].to_vec();
        if !self.server.settings.gui {
            self.server_args.push("--nogui");
        }
        let server = &self.server;
        let mut command = std::process::Command::new("java");
        command
            .current_dir(&server.location)
            .args(default_args)
            .args(&self.java_args)
            .arg("-jar")
            .arg(&server.get_jar_path()?)
            .args(&self.server_args)
            .stdout(Stdio::null());
        command.spawn().map_err(Error::from)
    }
}

