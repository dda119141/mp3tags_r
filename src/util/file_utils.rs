use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write};

use crate::Result;

/// Create a temporary file path based on the original file path
pub fn create_temp_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    let file_name = path.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"))?;
    
    let mut temp_name = format!(".{}", file_name.to_string_lossy());
    temp_name.push_str(".tmp");
    
    let temp_path = path.with_file_name(temp_name);
    
    Ok(temp_path)
}

/// Replace a file with another file
pub fn replace_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    fs::rename(src, dst)?;
    Ok(())
}

/// Copy data from one file to another
pub fn copy_data(src: &mut File, dst: &mut File, size: usize) -> Result<()> {
    const BUFFER_SIZE: usize = 8192;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut remaining = size;
    
    while remaining > 0 {
        let to_read = std::cmp::min(BUFFER_SIZE, remaining);
        let read = src.read(&mut buffer[..to_read])?;
        
        if read == 0 {
            break;
        }
        
        dst.write_all(&buffer[..read])?;
        remaining -= read;
    }
    
    Ok(())
}
