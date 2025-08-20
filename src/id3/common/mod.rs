use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::Result;

pub const ID3V1_TAG_SIZE: usize = 128;
pub const ID3V1_IDENTIFIER: &[u8] = b"TAG";

/// Check if a file has an ID3v1 tag
pub fn has_id3v1_tag<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();

    if file_size < ID3V1_TAG_SIZE as u64 {
        return Ok(false);
    }

    file.seek(SeekFrom::End(-(ID3V1_TAG_SIZE as i64)))?;
    let mut buffer = [0u8; 3];
    file.read_exact(&mut buffer)?;

    Ok(buffer == ID3V1_IDENTIFIER)
}
