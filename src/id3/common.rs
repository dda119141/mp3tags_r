use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::Result;


/// Constants for ID3 tags
pub mod constants {
    /// ID3v1 tag size
    pub const ID3V1_TAG_SIZE: usize = 128;
    
    /// ID3v1 tag identifier
    pub const ID3V1_IDENTIFIER: &[u8] = b"TAG";
    
    /// ID3v2 tag identifier
    pub const ID3V2_IDENTIFIER: &[u8] = b"ID3";
    
    /// ID3v2 tag header size
    pub const ID3V2_HEADER_SIZE: usize = 10;
    
    /// ID3v2 frame header size (ID3v2.3 and ID3v2.4)
    pub const ID3V2_FRAME_HEADER_SIZE: usize = 10;
    
    /// ID3v2 flag for extended header
    pub const ID3V2_FLAG_EXTENDED_HEADER: u8 = 0x40;
    
    /// ID3v2 padding size
    pub const ID3V2_PADDING_SIZE: usize = 2048;
}

/// Checks if a file has an ID3v1 tag
pub fn has_id3v1_tag<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    
    if file_size < constants::ID3V1_TAG_SIZE as u64 {
        return Ok(false);
    }
    
    file.seek(SeekFrom::End(-(constants::ID3V1_TAG_SIZE as i64)))?;
    
    let mut tag_id = [0u8; 3];
    file.read_exact(&mut tag_id)?;
    
    Ok(&tag_id == constants::ID3V1_IDENTIFIER)
}

/// Converts a synchsafe integer to a normal integer
pub fn synchsafe_to_int(bytes: &[u8; 4]) -> u32 {
    ((bytes[0] as u32 & 0x7F) << 21) |
    ((bytes[1] as u32 & 0x7F) << 14) |
    ((bytes[2] as u32 & 0x7F) << 7) |
    (bytes[3] as u32 & 0x7F)
}

/// Converts a normal integer to a synchsafe integer
pub fn int_to_synchsafe(value: u32) -> [u8; 4] {
    [
        ((value >> 21) & 0x7F) as u8,
        ((value >> 14) & 0x7F) as u8,
        ((value >> 7) & 0x7F) as u8,
        (value & 0x7F) as u8,
    ]
}
