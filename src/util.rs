use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::Error;
use crate::Result;

/// The modified file extension for temporary files
pub const MODIFIED_ENDING: &str = ".mod";

/// Reads a file into a buffer
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Writes a buffer to a file
pub fn write_file<P: AsRef<Path>>(path: P, buffer: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(buffer)?;
    Ok(())
}

/// Renames a file, handling errors
pub fn rename_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    fs::rename(&from, &to).map_err(|e| Error::FileRenameError(e.to_string()))
}

/// Creates a temporary path for a file
pub fn get_temp_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut temp_path = path.to_path_buf();
    let extension = path.extension().map_or_else(|| "tmp".to_string(), |ext| {
        format!("{}.tmp", ext.to_string_lossy())
    });
    temp_path.set_extension(extension);
    temp_path
}

/// Copies a range of bytes from one file to another
pub fn copy_file_range(source: &mut File, target: &mut File) -> Result<()> {
    const BUFFER_SIZE: usize = 8192;
    let mut buffer = [0u8; BUFFER_SIZE];
    
    loop {
        let bytes_read = source.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        target.write_all(&buffer[..bytes_read])?;
    }
    
    Ok(())
}

/// Gets the absolute path of a file
pub fn absolute_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    Ok(absolute)
}

/// Extracts a string from a buffer at a given position and length
pub fn extract_string(buffer: &[u8], start: usize, length: usize) -> Result<String> {
    if start + length > buffer.len() {
        return Err(Error::Other(format!(
            "Buffer size {} < requested length: {}",
            buffer.len(),
            start + length
        )));
    }

    let bytes = &buffer[start..start + length];
    
    // Filter out non-printable characters
    let filtered: Vec<u8> = bytes
        .iter()
        .filter(|&&b| b >= 32 && b <= 126)
        .cloned()
        .collect();
    
    String::from_utf8(filtered)
        .map_err(|_| Error::NonPrintableContent)
}

/// Gets the tag size from a buffer using specified parameters
pub fn get_tag_size(buffer: &[u8], start: usize, length: usize, big_endian: bool) -> Result<u32> {
    if start + length > buffer.len() {
        return Err(Error::InvalidTagSize);
    }

    let mut size = 0u32;
    let bytes = &buffer[start..start + length];

    if big_endian {
        for (i, &byte) in bytes.iter().enumerate() {
            size |= (byte as u32) << ((length - 1 - i) * 8);
        }
    } else {
        for (i, &byte) in bytes.iter().enumerate() {
            size |= (byte as u32) << (i * 8);
        }
    }

    Ok(size)
}

/// Updates a size field in a buffer
pub fn update_size_field(buffer: &mut [u8], start: usize, length: usize, extra_size: u32, big_endian: bool) -> Result<()> {
    if start + length > buffer.len() {
        return Err(Error::InvalidTagSize);
    }

    let mut current_size = get_tag_size(buffer, start, length, big_endian)?;
    current_size += extra_size;

    let bytes = &mut buffer[start..start + length];
    
    if big_endian {
        for i in 0..length {
            bytes[i] = ((current_size >> ((length - 1 - i) * 8)) & 0xFF) as u8;
        }
    } else {
        for i in 0..length {
            bytes[i] = ((current_size >> (i * 8)) & 0xFF) as u8;
        }
    }

    Ok(())
}

/// Searches for a pattern in a buffer
pub fn search_pattern(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }

    for i in 0..=haystack.len() - needle.len() {
        if haystack[i..i + needle.len()] == needle[..] {
            return Some(i);
        }
    }

    None
}
