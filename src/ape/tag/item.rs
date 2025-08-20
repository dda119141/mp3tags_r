use std::io::{Read, Write};
use crate::error::Result;

/// APE item flags
#[derive(Debug, Clone, Copy)]
pub struct ItemFlags {
    pub read_only: bool,
    pub binary: bool,
    pub external: bool,
}

/// APE tag item implementation
#[derive(Debug)]
pub struct Item {
    pub key: String,
    pub value: Vec<u8>,
    pub flags: ItemFlags,
}

impl Item {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            value: Vec::new(),
            flags: ItemFlags::default(),
        }
    }

    pub fn write_to_file(&self, writer: &mut impl Write) -> Result<()> {
        let value_len = self.value.len() as u32;
        let flags = self.flags_as_u32();
        let key_bytes = self.key.as_bytes();
        
        writer.write_all(&value_len.to_le_bytes())?;
        writer.write_all(&flags.to_le_bytes())?;
        writer.write_all(key_bytes)?;
        writer.write_all(&[0])?; // null terminator
        writer.write_all(&self.value)?;
        
        Ok(())
    }

    fn flags_as_u32(&self) -> u32 {
        let mut flags = 0u32;
        if self.flags.read_only { flags |= 1 }
        if self.flags.binary { flags |= 2 }
        if self.flags.external { flags |= 4 }
        flags
    }
}

impl Default for ItemFlags {
    fn default() -> Self {
        Self {
            read_only: false,
            binary: false,
            external: false,
        }
    }
}
