/// Convert a synchsafe integer to a regular u32.
pub fn synchsafe_to_int(bytes: &[u8]) -> u32 {
    let mut result = 0u32;
    for byte in bytes {
        result = (result << 7) | (*byte as u32 & 0x7F);
    }
    result
}

/// Convert a u32 to a synchsafe integer byte array.
pub fn int_to_synchsafe(val: u32) -> [u8; 4] {
    let mut bytes = [0u8; 4];
    bytes[0] = ((val >> 21) & 0x7F) as u8;
    bytes[1] = ((val >> 14) & 0x7F) as u8;
    bytes[2] = ((val >> 7) & 0x7F) as u8;
    bytes[3] = (val & 0x7F) as u8;
    bytes
}

use std::io::Read;

pub fn has_id3v2_tag(path: &std::path::Path) -> crate::Result<bool> {
    let mut file = std::fs::File::open(path)?;
    let mut header = [0; 10];
    if file.read(&mut header)? < 10 {
        return Ok(false);
    }
    Ok(&header[0..3] == crate::id3::constants::ID3V2_IDENTIFIER)
}
