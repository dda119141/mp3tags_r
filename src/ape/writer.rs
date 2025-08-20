use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::collections::HashMap;
use crate::TagType;

use crate::Result;
use crate::Error;
use crate::MetaEntry;
use crate::tag::TagWriterStrategy;
use crate::util;
use crate::ape::common::{constants, has_ape_tag};
use crate::ape::reader::{ApeReader, ApeTag};

/// APE tag writers
#[derive(Debug, Default)]
pub struct ApeWriter {
    path: Option<PathBuf>,
    tag: Option<ApeTag>,
}

/// Convert MetaEntry to APE tag key
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

/// Check if file has ID3v1 tag and return the tag data if present
fn check_id3v1_tag(file: &mut File, file_size: u64) -> Result<Option<[u8; 128]>> {
    if file_size < 128 {
        return Ok(None);
    }
    
    let mut id3v1_tag = [0u8; 128];
    file.seek(SeekFrom::End(-128))?;
    file.read_exact(&mut id3v1_tag)?;
    
    if &id3v1_tag[0..3] == b"TAG" {
        Ok(Some(id3v1_tag))
    } else {
        Ok(None)
    }
}

impl ApeWriter {
    /// Create a new APE tag writer
    pub fn new() -> Self {
        Self {
            path: None,
            tag: None,
        }
    }
    
    /// Write APE tag to a file
    pub fn write_tag<P: AsRef<Path>>(&self, path: P, tag: &ApeTag) -> Result<()> {
        let path = path.as_ref();
        
        // Create a temporary file
        let temp_path = util::get_temp_path(path);
        let mut temp_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_path)?;
        
        // Open the original file for reading
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        
        // Check for ID3v1 tag
        let id3v1_tag = check_id3v1_tag(&mut file, file_size)?;
        
        // Copy audio data to the temporary file
        file.seek(SeekFrom::Start(0))?;
        util::copy_file_range(&mut file, &mut temp_file)?;
        
        // Write APE tag header if present
        if let Some(header) = &tag.header {
            let mut header_buffer = [0u8; constants::APE_TAG_HEADER_SIZE];
            header.to_buffer(&mut header_buffer)?;
            temp_file.write_all(&header_buffer)?;
        }
        
        // Write APE tag items
        for item in &tag.items {
            // Write size and flags
            temp_file.write_all(&item.size.to_le_bytes())?;
            temp_file.write_all(&item.flags.to_le_bytes())?;
            
            // Write key (null-terminated)
            temp_file.write_all(item.key.as_bytes())?;
            temp_file.write_all(&[0])?;
            
            // Write value
            temp_file.write_all(&item.value)?;
        }
        
        // Write APE tag footer
        let mut footer_buffer = [0u8; constants::APE_TAG_FOOTER_SIZE];
        tag.footer.to_buffer(&mut footer_buffer)?;
        temp_file.write_all(&footer_buffer)?;
        
        // Write ID3v1 tag if present
        if let Some(id3v1_data) = id3v1_tag {
            temp_file.write_all(&id3v1_data)?;
        }
        
        // Replace the original file with the temporary file
        util::rename_file(&temp_path, path)?;
        
        Ok(())
    }
    
    /// Remove APE tag from a file
    pub fn remove_tag<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        // Check if the file has an APE tag
        if !has_ape_tag(path)? {
            return Ok(());
        }
        
        // Create a temporary file
        let temp_path = util::get_temp_path(path);
        let mut temp_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_path)?;
        
        // Open the original file for reading
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        
        // Check for ID3v1 tag
        let id3v1_tag = check_id3v1_tag(&mut file, file_size)?;
        
        // Copy audio data to the temporary file
        file.seek(SeekFrom::Start(0))?;
        util::copy_file_range(&mut file, &mut temp_file)?;
        
        // Write ID3v1 tag if present
        if let Some(id3v1_data) = id3v1_tag {
            temp_file.write_all(&id3v1_data)?;
        }
        
        // Replace the original file with the temporary file
        util::rename_file(&temp_path, path)?;
        
        Ok(())
    }
    
    /// Set meta entries in a file
    pub fn set_meta_entries<P: AsRef<Path>>(&self, path: P, entries: &HashMap<MetaEntry, String>) -> Result<()> {
        let path = path.as_ref();
        
        // Read existing tag or create a new one
        let reader = ApeReader::new();
        let mut tag = match reader.read_tag(path) {
            Ok(tag) => tag,
            Err(Error::TagNotFound) => ApeTag::new(constants::APE_TAG_VERSION_2_0),
            Err(e) => return Err(e),
        };
        
        // Update tag with new entries
        for (entry, value) in entries {
            let key = meta_entry_to_ape_key(entry);
            tag.set_text_item(key, value);
        }
        
        // Write the updated tag
        self.write_tag(path, &tag)
    }
    
    /// Remove meta entries from a file
    pub fn remove_meta_entries<P: AsRef<Path>>(&self, path: P, entries: &[MetaEntry]) -> Result<()> {
        let path = path.as_ref();
        
        // Read existing tag
        let reader = ApeReader::new();
        let mut tag = match reader.read_tag(path) {
            Ok(tag) => tag,
            Err(Error::TagNotFound) => return Ok(()), // No tag to remove entries from
            Err(e) => return Err(e),
        };
        
        // Remove entries
        for entry in entries {
            let key = meta_entry_to_ape_key(entry);
            tag.remove_item(key);
        }
        
        // If no items left, remove the tag
        if tag.items.is_empty() {
            self.remove_tag(path)
        } else {
            // Write the updated tag
            self.write_tag(path, &tag)
        }
    }
}

impl TagWriterStrategy for ApeWriter {
    fn init(&mut self, _path: &Path) -> Result<()> {
        // No initialization needed for APE writer
        Ok(())
    }
    
    fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()> {
        if let Some(tag) = &mut self.tag {
            let key = meta_entry_to_ape_key(entry);
            tag.set_text_item(key, value);
            Ok(())
        } else {
            Err(Error::TagNotFound)
        }
    }
    
    fn save(&mut self) -> Result<()> {
        if let Some(tag) = &self.tag {
            if let Some(path) = &self.path {
                tag.write_to_file(path)
            } else {
                Err(Error::Other("No path set for APE writer".to_string()))
            }
        } else {
            Err(Error::TagNotFound)
        }
    }
    
    fn tag_type(&self) -> TagType {
        TagType::Ape
    }
}
