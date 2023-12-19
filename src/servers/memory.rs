pub enum Memory {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl Memory {
    pub fn to_string(&self) -> String {
        match self {
            Memory::Bytes(bytes) => format!("{}B", bytes),
            Memory::KiloBytes(kb) => format!("{}KB", kb),
            Memory::MegaBytes(mb) => format!("{}MB", mb),
            Memory::GigaBytes(gb) => format!("{}GB", gb),
        }
    }
}