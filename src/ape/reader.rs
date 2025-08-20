use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashMap;

use crate::Result;
use crate::Error;
use crate::MetaEntry;
use crate::tag::TagReaderStrategy;
use crate::TagType;
use crate::ape::common::{constants, ApeTagHeader, ApeItem};

/// Convert MetaEntry to APE tag key (shared with writer)
fn meta_entry_to_ape_key(entry: &MetaEntry) -> &str {
    match entry {
        MetaEntry::Title => "TITLE",
        MetaEntry::Artist => "ARTIST",
        MetaEntry::Album => "ALBUM",
        MetaEntry::Year => "YEAR",
        MetaEntry::Genre => "GENRE",
        MetaEntry::Comment => "COMMENT",
        MetaEntry::Composer => "COMPOSER",
        MetaEntry::Track => "TRACK",
        MetaEntry::Date => "DATE",
        MetaEntry::TextWriter => "TEXTWRITER",
        MetaEntry::AudioEncryption => "AUDIOENCRYPTION",
        MetaEntry::Language => "LANGUAGE",
        MetaEntry::Time => "TIME",
        MetaEntry::OriginalFilename => "ORIGINALFILENAME",
        MetaEntry::FileType => "FILETYPE",
        MetaEntry::BandOrchestra => "BANDORCHESTRA",
        MetaEntry::Custom(key) => key,
    }
}

// ============================================================================
// APE Tag Data Structure
// ============================================================================

/// APE tag structure
#[derive(Debug, Clone)]
pub struct ApeTag {
    /// Tag header (optional)
    pub header: Option<ApeTagHeader>,
    /// Tag footer
    pub footer: ApeTagHeader,
    /// Tag items
    pub items: Vec<ApeItem>,
}

impl ApeTag {
    /// Create a new APE tag
    pub fn new(version: u32) -> Self {
        let footer_flags = constants::flags::APE_TAG_FLAG_HAS_HEADER;
        let header_flags = constants::flags::APE_TAG_FLAG_HAS_HEADER | constants::flags::APE_TAG_FLAG_IS_HEADER;
        
        let footer = ApeTagHeader::new(version, constants::APE_TAG_FOOTER_SIZE as u32, 0, footer_flags);
        let header = ApeTagHeader::new(version, constants::APE_TAG_FOOTER_SIZE as u32, 0, header_flags);
        
        Self {
            header: Some(header),
            footer,
            items: Vec::new(),
        }
    }
    
    // ------------------------------------------------------------------------
    // Core Item Access Methods
    // ------------------------------------------------------------------------
    
    /// Get an item by key
    pub fn get_item(&self, key: &str) -> Option<&ApeItem> {
        self.items.iter().find(|item| item.key.eq_ignore_ascii_case(key))
    }
    
    /// Get a text item value by key
    pub fn get_item_text(&self, key: &str) -> Result<Option<String>> {
        let item = match self.get_item(key) {
            Some(item) => item,
            None => return Ok(None),
        };

        self.validate_text_item(item)?;
        self.item_value_to_string(item).map(Some)
    }

    /// Validate that an item is a text item (not binary)
    fn validate_text_item(&self, item: &ApeItem) -> Result<()> {
        if item.flags & constants::item_flags::APE_ITEM_FLAG_BINARY != 0 {
            return Err(Error::Other("Item is binary, not text".to_string()));
        }
        Ok(())
    }

    /// Convert item value bytes to UTF-8 string
    fn item_value_to_string(&self, item: &ApeItem) -> Result<String> {
        String::from_utf8(item.value.clone())
            .map_err(|_| Error::Other("Invalid UTF-8 data".to_string()))
    }
    
    // ------------------------------------------------------------------------
    // Item Modification Methods
    // ------------------------------------------------------------------------
    
    /// Add or update an item
    pub fn set_item(&mut self, item: ApeItem) {
        if let Some(index) = self.items.iter().position(|i| i.key.eq_ignore_ascii_case(&item.key)) {
            self.items[index] = item;
        } else {
            self.items.push(item);
        }
        
        self.update_size_and_count();
    }
    
    /// Set a text item
    pub fn set_text_item(&mut self, key: &str, value: &str) {
        // Find existing item or add new one
        if let Some(index) = self.items.iter().position(|i| i.key.eq_ignore_ascii_case(key)) {
            // Update existing item
            let item = ApeItem::new_text(key, value);
            self.items[index] = item;
        } else {
            // Add new item
            let item = ApeItem::new_text(key, value);
            self.items.push(item);
        }
        
        // Update tag size and item count
        let mut total_size = constants::APE_TAG_FOOTER_SIZE;
        if self.header.is_some() {
            total_size += constants::APE_TAG_HEADER_SIZE;
        }
        
        for item in &self.items {
            total_size += item.total_size() as usize;
        }
        
        self.footer.item_count = self.items.len() as u32;
        self.footer.size = total_size as u32;
        
        if let Some(header) = &mut self.header {
            header.item_count = self.items.len() as u32;
            header.size = total_size as u32;
        }
    }
    
    /// Remove an item by key
    pub fn remove_item(&mut self, key: &str) -> bool {
        let len_before = self.items.len();
        self.items.retain(|item| !item.key.eq_ignore_ascii_case(key));
        let removed = len_before > self.items.len();
        
        if removed {
            self.update_size_and_count();
        }
        
        removed
    }
    
    // ------------------------------------------------------------------------
    // MetaEntry Interface Methods
    // ------------------------------------------------------------------------
    
    /// Get all meta entries
    pub fn get_meta_entries(&self) -> HashMap<MetaEntry, String> {
        let mut entries = HashMap::new();
        
        for item in &self.items {
            if let Ok(text) = item.get_text() {
                let key = &item.key;
                
                // Try to map to standard MetaEntry first
                let meta_entry = match key.to_uppercase().as_str() {
                    "TITLE" => MetaEntry::Title,
                    "ARTIST" => MetaEntry::Artist,
                    "ALBUM" => MetaEntry::Album,
                    "YEAR" => MetaEntry::Year,
                    "GENRE" => MetaEntry::Genre,
                    "COMMENT" => MetaEntry::Comment,
                    "COMPOSER" => MetaEntry::Composer,
                    "TRACK" => MetaEntry::Track,
                    "DATE" => MetaEntry::Date,
                    "TEXTWRITER" => MetaEntry::TextWriter,
                    "AUDIOENCRYPTION" => MetaEntry::AudioEncryption,
                    "LANGUAGE" => MetaEntry::Language,
                    "TIME" => MetaEntry::Time,
                    "ORIGINALFILENAME" => MetaEntry::OriginalFilename,
                    "FILETYPE" => MetaEntry::FileType,
                    "BANDORCHESTRA" => MetaEntry::BandOrchestra,
                    _ => MetaEntry::Custom(key.clone()),
                };
                
                entries.insert(meta_entry, text);
            }
        }
        
        entries
    }
    
    /// Set a meta entry
    pub fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()> {
        let key = meta_entry_to_ape_key(entry);
        self.set_text_item(key, value);
        Ok(())
    }
    
    // ------------------------------------------------------------------------
    // File I/O Methods
    // ------------------------------------------------------------------------
    
    /// Write the tag to a file
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use crate::ape::writer::ApeWriter;
        let writer = ApeWriter::new();
        writer.write_tag(path, self)
    }
    
    // ------------------------------------------------------------------------
    // Private Helper Methods
    // ------------------------------------------------------------------------
    
    /// Update tag size and item count after modifications
    fn update_size_and_count(&mut self) {
        let mut total_size = constants::APE_TAG_FOOTER_SIZE;
        if self.header.is_some() {
            total_size += constants::APE_TAG_HEADER_SIZE;
        }
        
        for item in &self.items {
            total_size += item.total_size() as usize;
        }
        
        self.footer.item_count = self.items.len() as u32;
        self.footer.size = total_size as u32;
        
        if let Some(header) = &mut self.header {
            header.item_count = self.items.len() as u32;
            header.size = total_size as u32;
        }
    }
}

// ============================================================================
// APE Tag Reader
// ============================================================================

/// APE tag reader
#[derive(Debug, Default)]
pub struct ApeReader;

impl ApeReader {
    /// Create a new APE tag reader
    pub fn new() -> Self {
        Self
    }
    
    /// Read APE tag from a file
    pub fn read_tag<P: AsRef<Path>>(&self, path: P) -> Result<ApeTag> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        
        if file_size < constants::APE_TAG_FOOTER_SIZE as u64 {
            return Err(Error::TagNotFound);
        }
        
        // Try APE tag at end of file
        if let Some(footer) = self.try_read_footer_at(&mut file, -(constants::APE_TAG_FOOTER_SIZE as i64))? {
            return self.read_tag_with_footer(&mut file, footer);
        }
        
        // Try APE tag before ID3v1 tag
        if file_size >= (constants::APE_TAG_FOOTER_SIZE + 128) as u64 {
            if let Some(footer) = self.try_read_footer_at(&mut file, -((constants::APE_TAG_FOOTER_SIZE + 128) as i64))? {
                return self.read_tag_with_footer(&mut file, footer);
            }
        }
        
        Err(Error::TagNotFound)
    }
    
    // ------------------------------------------------------------------------
    // Private Helper Methods
    // ------------------------------------------------------------------------
    
    /// Try to read APE footer at given position
    fn try_read_footer_at(&self, file: &mut File, offset: i64) -> Result<Option<ApeTagHeader>> {
        file.seek(SeekFrom::End(offset))?;
        let mut footer_buffer = [0u8; constants::APE_TAG_FOOTER_SIZE];
        file.read_exact(&mut footer_buffer)?;
        
        match ApeTagHeader::from_buffer(&footer_buffer) {
            Ok(footer) => Ok(Some(footer)),
            Err(_) => Ok(None),
        }
    }
    
    /// Read APE tag with known footer
    fn read_tag_with_footer(&self, file: &mut File, footer: ApeTagHeader) -> Result<ApeTag> {
        self.seek_to_tag_data(file, &footer)?;

        let header = self.read_header_if_present(file, &footer)?;
        let items = self.read_items(file, footer.item_count as usize)?;

        Ok(ApeTag {
            header,
            footer,
            items,
        })
    }

    fn seek_to_tag_data(&self, file: &mut File, footer: &ApeTagHeader) -> Result<u64> {
        let tag_size = footer.size as i64;
        let seek_offset = if footer.has_header() {
            -(tag_size + constants::APE_TAG_HEADER_SIZE as i64)
        } else {
            -tag_size
        };
        Ok(file.seek(SeekFrom::End(seek_offset))?)
    }

    fn read_header_if_present(&self, file: &mut File, footer: &ApeTagHeader) -> Result<Option<ApeTagHeader>> {
        if !footer.has_header() {
            return Ok(None);
        }

        let mut header_buffer = [0u8; constants::APE_TAG_HEADER_SIZE];
        file.read_exact(&mut header_buffer)?;

        let header = ApeTagHeader::from_buffer(&header_buffer)?;
        if !header.is_header() {
            return Err(Error::Other("Invalid APE tag header".to_string()));
        }

        Ok(Some(header))
    }

    fn read_items(&self, file: &mut File, item_count: usize) -> Result<Vec<ApeItem>> {
        let mut items = Vec::with_capacity(item_count);
        for _ in 0..item_count {
            items.push(self.read_item(file)?);
        }
        Ok(items)
    }

    fn read_item(&self, file: &mut File) -> Result<ApeItem> {
        const MAX_KEY_LENGTH: usize = 255; // APE spec limit
        const MAX_VALUE_SIZE: usize = 16 * 1024 * 1024; // 16MB reasonable limit
        
        let mut size_flags_buffer = [0u8; 8];
        file.read_exact(&mut size_flags_buffer)?;

        let size = u32::from_le_bytes(size_flags_buffer[0..4].try_into().unwrap());
        let flags = u32::from_le_bytes(size_flags_buffer[4..8].try_into().unwrap());

        // Security check: prevent excessive memory allocation
        if size as usize > MAX_VALUE_SIZE {
            return Err(Error::Other(format!("APE item value too large: {} bytes", size)));
        }

        // Read key bytes until null terminator with length limit
        let mut key_bytes = Vec::new();
        for _ in 0..MAX_KEY_LENGTH {
            let mut byte = [0u8; 1];
            file.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            key_bytes.push(byte[0]);
        }
        
        // Security check: ensure we found null terminator
        if key_bytes.len() >= MAX_KEY_LENGTH {
            return Err(Error::Other("APE item key too long or missing null terminator".to_string()));
        }

        let key = String::from_utf8(key_bytes)
            .map_err(|_| Error::Other("Invalid UTF-8 in APE item key".to_string()))?;

        let mut value = vec![0u8; size as usize];
        file.read_exact(&mut value)?;

        Ok(ApeItem {
            size,
            flags,
            key,
            value,
        })
    }
}

// ============================================================================
// TagReaderStrategy Implementation
// ============================================================================

impl TagReaderStrategy for ApeReader {
    fn init(&mut self, _path: &Path) -> Result<()> {
        // No initialization needed for APE reader
        Ok(())
    }
    
    fn get_meta_entry(&self, path: &Path, entry: &MetaEntry) -> Result<Option<String>> {
        match self.read_tag(path) {
            Ok(tag) => {
                let key = meta_entry_to_ape_key(entry);
                tag.get_item_text(key)
            },
            Err(Error::TagNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    fn tag_type(&self) -> TagType {
        TagType::Ape
    }
}
