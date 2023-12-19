use crate::servers::memory::Memory;

/// Represents the settings of a server
pub struct ServerSettings {
    pub gui: bool,
    pub initial_memory: Memory,
    pub max_memory: Memory,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            gui: true, // If false and no support for --no-gui, arg isn't recognized
            initial_memory: Memory::GigaBytes(1),
            max_memory: Memory::GigaBytes(2),
        }
    }
}