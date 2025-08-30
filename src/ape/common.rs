use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::error::{Error, Result};

/// Constants for APE tags
pub mod constants {
    /// APE tag footer size
    pub const APE_TAG_FOOTER_SIZE: usize = 32;
    
    /// APE tag header size (same as footer)
    pub const APE_TAG_HEADER_SIZE: usize = 32;
    
    /// APE tag identifier
    pub const APE_TAG_IDENTIFIER: &[u8] = b"APETAGEX";
    
    /// APE tag version 2.0
    pub const APE_TAG_VERSION_2_0: u32 = 2000;
    
    /// APE tag flags
    pub mod flags {
        /// Tag contains a header
        pub const APE_TAG_FLAG_HAS_HEADER: u32 = 1 << 31;
        
        /// Tag contains no footer
        pub const APE_TAG_FLAG_NO_FOOTER: u32 = 1 << 30;
        
        /// This is the header, not the footer
        pub const APE_TAG_FLAG_IS_HEADER: u32 = 1 << 29;
    }
    
    /// APE item flags
    pub mod item_flags {
        /// Item contains binary data
        pub const APE_ITEM_FLAG_BINARY: u32 = 2;
        
        /// Item contains UTF-8 text
        pub const APE_ITEM_FLAG_UTF8: u32 = 0;
    }
}

/// APE tag header/footer structure
#[derive(Debug, Clone)]
pub struct ApeTagHeader {
    /// Tag identifier ("APETAGEX")
    pub identifier: [u8; 8],
    /// Tag version (1000 = 1.0, 2000 = 2.0)
    pub version: u32,
    /// Tag size (including footer, excluding header)
    pub size: u32,
    /// Number of items in the tag
    pub item_count: u32,
    /// Tag flags
    pub flags: u32,
    /// Reserved (should be 0)
    pub reserved: [u8; 8],
}

impl ApeTagHeader {
    /// Create a new APE tag header
    pub fn new(version: u32, size: u32, item_count: u32, flags: u32) -> Self {
        let mut identifier = [0u8; 8];
        identifier.copy_from_slice(constants::APE_TAG_IDENTIFIER);
        
        Self {
            identifier,
            version,
            size,
            item_count,
            flags,
            reserved: [0u8; 8],
        }
    }
    
    /// Read an APE tag header from a buffer
    pub fn from_buffer(buffer: &[u8]) -> Result<Self> {
        if buffer.len() < constants::APE_TAG_HEADER_SIZE {
            return Err(Error::Other("Buffer too small for APE tag header".to_string()));
        }
        
        let mut identifier = [0u8; 8];
        identifier.copy_from_slice(&buffer[0..8]);
        
        if identifier != constants::APE_TAG_IDENTIFIER {
            return Err(Error::TagNotFound);
        }
        
        // Parse header fields from little-endian bytes
        let version = u32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);     // APE version (1000 or 2000)
        let size = u32::from_le_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]);      // Tag size in bytes (excluding header/footer)
        let item_count = u32::from_le_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]); // Number of items in the tag
        let flags = u32::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]);     // Tag flags (header present, footer present, etc.)
        
        let mut reserved = [0u8; 8];
        reserved.copy_from_slice(&buffer[24..32]);
        
        Ok(Self {
            identifier,
            version,
            size,
            item_count,
            flags,
            reserved,
        })
    }
    
    /// Write the APE tag header to a buffer
    pub fn to_buffer(&self, buffer: &mut [u8]) -> Result<()> {
        if buffer.len() < constants::APE_TAG_HEADER_SIZE {
            return Err(Error::Other("Buffer too small for APE tag header".to_string()));
        }
        
        buffer[0..8].copy_from_slice(&self.identifier);
        buffer[8..12].copy_from_slice(&self.version.to_le_bytes());
        buffer[12..16].copy_from_slice(&self.size.to_le_bytes());
        buffer[16..20].copy_from_slice(&self.item_count.to_le_bytes());
        buffer[20..24].copy_from_slice(&self.flags.to_le_bytes());
        buffer[24..32].copy_from_slice(&self.reserved);
        
        Ok(())
    }
    
    /// Check if this is a header (not a footer)
    pub fn is_header(&self) -> bool {
        self.flags & constants::flags::APE_TAG_FLAG_IS_HEADER != 0
    }
    
    /// Check if the tag has a header
    pub fn has_header(&self) -> bool {
        self.flags & constants::flags::APE_TAG_FLAG_HAS_HEADER != 0
    }
    
    /// Check if the tag has a footer
    pub fn has_footer(&self) -> bool {
        self.flags & constants::flags::APE_TAG_FLAG_NO_FOOTER == 0
    }
}

/// APE tag item structure
#[derive(Debug, Clone)]
pub struct ApeItem {
    /// Item value size
    pub size: u32,
    /// Item flags
    pub flags: u32,
    /// Item key
    pub key: String,
    /// Item value
    pub value: Vec<u8>,
}

impl ApeItem {
    /// Create a new APE item
    pub fn new(key: &str, value: Vec<u8>, flags: u32) -> Self {
        Self {
            size: value.len() as u32,
            flags,
            key: key.to_string(),
            value,
        }
    }
    
    /// Create a new text APE item
    pub fn new_text(key: &str, value: &str) -> Self {
        Self::new(key, value.as_bytes().to_vec(), constants::item_flags::APE_ITEM_FLAG_UTF8)
    }
    
    /// Get the size of the item (including key and value)
    pub fn total_size(&self) -> u32 {
        // Size + Flags + Key (null-terminated) + Value
        8 + self.key.len() as u32 + 1 + self.size
    }
    
    /// Get the text value of the item
    pub fn get_text(&self) -> Result<String> {
        if self.flags & constants::item_flags::APE_ITEM_FLAG_BINARY != 0 {
            return Err(Error::Other("Item is binary, not text".to_string()));
        }
        
        match String::from_utf8(self.value.clone()) {
            Ok(text) => Ok(text),
            Err(_) => Err(Error::Other("Invalid UTF-8 data".to_string())),
        }
    }
}

/// APE tag search location
#[derive(Debug, Clone, Copy)]
enum ApeTagLocation {
    /// At the end of the file
    EndOfFile,
    /// At the beginning of the file
    StartOfFile,
    /// Before ID3v1 tag (128 bytes from end)
    BeforeId3v1,
}

impl ApeTagLocation {
    /// Get the seek position for this location
    fn get_seek_position(&self, file_size: u64) -> Option<SeekFrom> {
        match self {
            ApeTagLocation::EndOfFile => {
                if file_size >= constants::APE_TAG_FOOTER_SIZE as u64 {
                    Some(SeekFrom::End(-(constants::APE_TAG_FOOTER_SIZE as i64)))
                } else {
                    None
                }
            }
            ApeTagLocation::StartOfFile => Some(SeekFrom::Start(0)),
            ApeTagLocation::BeforeId3v1 => {
                if file_size >= (constants::APE_TAG_FOOTER_SIZE + 128) as u64 {
                    Some(SeekFrom::End(-((constants::APE_TAG_FOOTER_SIZE + 128) as i64)))
                } else {
                    None
                }
            }
        }
    }
    
    /// Validate the found tag header for this location
    fn validate_header(&self, header: &ApeTagHeader) -> bool {
        match self {
            ApeTagLocation::StartOfFile => header.is_header(),
            _ => true, // Footer locations don't need special validation
        }
    }
}

/// Template function to check for APE tag at a specific location
fn check_ape_tag_at_location(file: &mut File, file_size: u64, location: ApeTagLocation) -> Result<bool> {
    if let Some(seek_pos) = location.get_seek_position(file_size) {
        file.seek(seek_pos)?;
        
        let mut buffer = [0u8; constants::APE_TAG_FOOTER_SIZE];
        file.read_exact(&mut buffer)?;
        
        if let Ok(tag_header) = ApeTagHeader::from_buffer(&buffer) {
            if location.validate_header(&tag_header) {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

/// Check if a file has an APE tag
pub fn has_ape_tag<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    
    // Define search locations in priority order
    let locations = [
        ApeTagLocation::EndOfFile,
        ApeTagLocation::StartOfFile,
        ApeTagLocation::BeforeId3v1,
    ];
    
    // Check each location using the template function
    for location in &locations {
        if check_ape_tag_at_location(&mut file, file_size, *location)? {
            return Ok(true);
        }
    }
    
    Ok(false)
}

