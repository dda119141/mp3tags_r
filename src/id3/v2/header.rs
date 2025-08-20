use crate::id3::v2::util::{int_to_synchsafe, synchsafe_to_int};
use crate::error::Result;

/// Extended header for ID3v2 tags
#[derive(Debug)]
pub struct ExtendedHeader {
    pub size: u32,
    pub flags: u16,
    pub padding_size: u32,
}

/// ID3v2 header implementation
#[derive(Debug)]
pub struct Header {
    pub version: u8,
    pub revision: u8,
    pub flags: u8,
    pub size: u32,
}


impl Header {
    pub fn new(version: u8) -> Self {
        Self {
            version,
            revision: 0,
            flags: 0,
            size: 0,
        }
    }

    pub fn parse(buffer: &[u8]) -> Result<Self> {
        if buffer.len() < 10 {
            return Err(crate::error::Error::InvalidHeader);
        }

        if &buffer[0..3] != b"ID3" {
            return Err(crate::error::Error::InvalidHeader);
        }

        Ok(Self {
            version: buffer[3],
            revision: buffer[4],
            flags: buffer[5],
            size: synchsafe_to_int(&[buffer[6], buffer[7], buffer[8], buffer[9]]),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(10);
        buffer.extend_from_slice(b"ID3");
        buffer.push(self.version);
        buffer.push(self.revision);
        buffer.push(self.flags);
        
        let size_bytes = int_to_synchsafe(self.size);
        buffer.extend_from_slice(&size_bytes);
        
        buffer
    }

    pub fn is_valid(&self) -> bool {
        self.version <= 4 && self.size > 0
    }
}

impl ExtendedHeader {
    pub fn new() -> Self {
        Self {
            size: 0,
            flags: 0,
            padding_size: 0,
        }
    }
}
