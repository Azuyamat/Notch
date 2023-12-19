use std::path::PathBuf;
use crate::jars::jar::Jar;
use crate::servers::memory::Memory;

pub struct Server {
    pub name: String,
    pub jar: Jar,
    pub location: PathBuf,
    pub settings: ServerSettings,
}

pub struct ServerSettings {
    pub gui: bool,
    pub initial_memory: Memory,
    pub max_memory: Memory,
}