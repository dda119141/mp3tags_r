use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::Result;

pub const ID3V2_TAG_IDENTIFIER: &[u8] = b"ID3";
pub const HEADER_SIZE: usize = 10;

/// Convert a synchsafe integer to a normal integer
pub fn synchsafe_to_int(bytes: [u8; 4]) -> u32 {
    let mut result = 0u32;
    result |= ((bytes[0] as u32) & 0x7F) << 21;
    result |= ((bytes[1] as u32) & 0x7F) << 14;
    result |= ((bytes[2] as u32) & 0x7F) << 7;
    result |= (bytes[3] as u32) & 0x7F;
    result
}

/// Convert a normal integer to a synchsafe integer
pub fn int_to_synchsafe(value: u32) -> [u8; 4] {
    let mut result = [0u8; 4];
    result[0] = ((value >> 21) & 0x7F) as u8;
    result[1] = ((value >> 14) & 0x7F) as u8;
    result[2] = ((value >> 7) & 0x7F) as u8;
    result[3] = (value & 0x7F) as u8;
    result
}

/// Check if a file has an ID3v2 tag
pub fn has_id3v2_tag<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 3];
    file.read_exact(&mut buffer)?;
    Ok(buffer == ID3V2_TAG_IDENTIFIER)
}
