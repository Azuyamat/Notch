use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// Represents a memory flag for the JVM.
#[derive(Deserialize, Serialize)]
pub enum Memory {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl Memory {
    pub(crate) fn prepend_flag(&self, flag: &str) -> String {
        format!("{}{}", flag, self)
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Memory::Bytes(bytes) => write!(f, "{bytes}B"),
            Memory::KiloBytes(kilobytes) => write!(f, "{kilobytes}K"),
            Memory::MegaBytes(megabytes) => write!(f, "{megabytes}M"),
            Memory::GigaBytes(gigabytes) => write!(f, "{gigabytes}G"),
        }
    }
}