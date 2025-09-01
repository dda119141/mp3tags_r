use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::{Result, MetaEntry, Error};
use crate::file_access::{FileManager};

/// Represents the type of tag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    /// ID3v1 tag
    Id3v1,
    /// ID3v2 tag
    Id3v2,
    /// APE tag
    Ape,
}

/// Simple trait for tag readers
pub trait TagReaderStrategy {
    /// Initialize the tag reader
    fn init(&mut self, path: &Path) -> Result<()>;
        
    /// Get a meta entry from the tag
    fn get_meta_entry(&self, path: &Path, entry: &MetaEntry) -> Result<String>;
    
    /// Get the tag type
    fn tag_type(&self) -> TagType;
}

/// Simple trait for tag writers
pub trait TagWriterStrategy {
    /// Initialize the tag writer
    fn init(&mut self, path: &Path) -> Result<()>;
    
    /// Set a meta entry in the tag
    fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()>;
    
    /// Save changes to the tag
    fn save(&mut self) -> Result<()>;
    
    /// Get the tag type
    fn tag_type(&self) -> TagType;
}

struct ReaderStrategy {
    selected: Box<dyn TagReaderStrategy>,
    initialized: bool,
}

struct WriterStrategy {
    selected: Box<dyn TagWriterStrategy>,
    initialized: bool,
}

/// Main tag reader class that uses the strategy pattern
pub struct TagReader {
    path: PathBuf,

    //pair of strategy and initialized flag
    strategies: Vec<ReaderStrategy>
}

impl TagReader {
    /// Create a new tag reader for the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        // Create file manager and validate file
        let file_manager = FileManager::with_default_strategy();
        file_manager.validate_file_path(&path)?;
        
        // Create strategies in order of preference
        let mut strategies: Vec<ReaderStrategy> = vec![
            ReaderStrategy { selected: Box::new(crate::id3::v2::tag::TagReader::new()), initialized: false },
            ReaderStrategy { selected: Box::new(crate::id3::v1::tag::TagReader::new()), initialized: false },
            ReaderStrategy { selected: Box::new(crate::ape::ApeReader::new()), initialized: false },
        ];
        
        // Initialize all strategies
        for strategy in &mut strategies {
            let handle = strategy.selected.init(&path);
            strategy.initialized = handle.is_ok();
        }
        
        Ok(Self { path, strategies })
    }
    
    /// Get a meta entry from the tag
    pub fn get_meta_entry(&self, entry: &MetaEntry) -> Result<String> {
        for strategy in &self.strategies {
            if strategy.initialized {
                if let Ok(value) = strategy.selected.get_meta_entry(&self.path, entry) {
                    return Ok(value);
                }
            }
        }
        Err(Error::EntryNotFound)
    }
      
    /// Get all meta entries from the tag
    pub fn get_all_meta_entries(&self) -> HashMap<MetaEntry, String> {
        let mut entries = HashMap::new();
        
        for entry in crate::meta_entry::all_standard_entries() {
            if let Ok(value) = self.get_meta_entry(&entry) {
                entries.insert(entry, value);
            }
        }
        
        entries
    }
}

/// Main tag writer class that uses the strategy pattern
pub struct TagWriter {
    strategies: Vec<WriterStrategy>,
    preferred_tag_type: TagType,
}

impl TagWriter {
    /// Create a new tag writer for the given path
    pub fn new<P: AsRef<Path>>(path: P, preferred_tag_type: TagType) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        // Create file manager and validate file
        let file_manager = FileManager::with_default_strategy();
        file_manager.validate_file_path(&path)?;
        
        // Create strategies in order of preference
        let mut strategies: Vec<WriterStrategy> = vec![
            WriterStrategy { selected: Box::new(crate::id3::v2::tag::TagWriter::new()), initialized: false },
            WriterStrategy { selected: Box::new(crate::id3::v1::tag::TagWriter::new()), initialized: false },
            WriterStrategy { selected: Box::new(crate::ape::ApeWriter::new()), initialized: false },
        ];
        
        // Initialize all strategies
        for strategy in &mut strategies {
            let handle = strategy.selected.init(&path);
            strategy.initialized = handle.is_ok();
        }
        
        Ok(Self {  
            strategies,
            preferred_tag_type,
        })
    }
    
    /// Set a meta entry in the tag
    pub fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()> {
        // First, try to find and use the preferred strategy if it's initialized.
        if let Some(strategy) = self.strategies.iter_mut().find(|s| s.initialized && 
                s.selected.tag_type() == self.preferred_tag_type) {
            return strategy.selected.set_meta_entry(entry, value);
        }

        // If the preferred strategy is not available or fails, try any other initialized strategy.
        for strategy in self.strategies.iter_mut().filter(|s| s.initialized) {
            if strategy.selected.set_meta_entry(entry, value).is_ok() {
                return Ok(());
            }
        }
        
        Err(Error::Other("Failed to set meta entry with any available strategy".to_string()))
    }
    
    /// Remove a meta entry from the tag
    pub fn remove_meta_entry(&mut self, entry: &MetaEntry) -> Result<()> {
        self.set_meta_entry(entry, "")
    }
    
    /// Remove multiple meta entries from the tag
    pub fn remove_meta_entries(&mut self, entries: &[MetaEntry]) -> Result<()> {
        for entry in entries {
            self.remove_meta_entry(entry)?;
        }
        Ok(())
    }
    
    /// Remove all meta entries from the tag
    pub fn remove_all_meta_entries(&mut self) -> Result<()> {
        let all_entries = crate::meta_entry::all_standard_entries();
        self.remove_meta_entries(&all_entries)
    }
}
// Convenience functions

/// Get the title of an MP3 file
pub fn get_title<P: AsRef<Path>>(path: P) -> Result<String> {
    let reader = TagReader::new(path)?;
    reader.get_meta_entry(&MetaEntry::Title)
}

/// Get the artist of an MP3 file
pub fn get_artist<P: AsRef<Path>>(path: P) -> Result<String> {
    let reader = TagReader::new(path)?;
    reader.get_meta_entry(&MetaEntry::Artist)
}

/// Get the album of an MP3 file
pub fn get_album<P: AsRef<Path>>(path: P) -> Result<String> {
    let reader = TagReader::new(path)?;
    reader.get_meta_entry(&MetaEntry::Album)
}

/// Get the year of an MP3 file
pub fn get_year<P: AsRef<Path>>(path: P) -> Result<String> {
    let reader = TagReader::new(path)?;
    reader.get_meta_entry(&MetaEntry::Year)
}

/// Get the genre of an MP3 file
pub fn get_genre<P: AsRef<Path>>(path: P) -> Result<String> {
    let reader = TagReader::new(path)?;
    reader.get_meta_entry(&MetaEntry::Genre)
}

/// Get the comment of an MP3 file
pub fn get_comment<P: AsRef<Path>>(path: P) -> Result<String> {
    let reader = TagReader::new(path)?;
    reader.get_meta_entry(&MetaEntry::Comment)
}

/// Get the composer of an MP3 file
pub fn get_composer<P: AsRef<Path>>(path: P) -> Result<String> {
    let reader = TagReader::new(path)?;
    reader.get_meta_entry(&MetaEntry::Composer)
}

/// Get all meta entries of an MP3 file
pub fn get_all_meta_entries<P: AsRef<Path>>(path: P) -> Result<HashMap<MetaEntry, String>> {
    let reader = TagReader::new(path)?;
    Ok(reader.get_all_meta_entries())
}
