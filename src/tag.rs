use std::path::{Path, PathBuf};
use std::collections::HashMap;

use crate::meta_entry::MetaEntry;
use crate::error::Error;
use crate::Result;

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

/// Tag presence information
#[derive(Debug, Clone, Copy, Default)]
pub struct TagPresence {
    /// ID3v1 tag is present
    pub id3v1_present: bool,
    /// ID3v2 tag is present
    pub id3v2_present: bool,
    /// APE tag is present
    pub ape_present: bool,
}


/// Trait for tag readers
pub trait TagReaderStrategy {
    /// Initialize the tag reader
    fn init(&mut self, path: &Path) -> Result<()>;
        
    /// Get a meta entry from the tag
    fn get_meta_entry(&self, path: &Path, entry: &MetaEntry) -> Result<Option<String>>;
    
    /// Get the tag type
    fn tag_type(&self) -> TagType;
}

/// Trait for tag writers
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

/// Enum representing different tag reading strategies
#[derive(Debug)]
pub enum ReaderStrategy {
    Id3v1(crate::id3::v1::tag::TagReader),
    Id3v2(crate::id3::v2::tag::TagReader),
    Ape(crate::ape::ApeReader),
}

impl ReaderStrategy {
    /// Get the tag type for this strategy
    pub fn tag_type(&self) -> TagType {
        match self {
            ReaderStrategy::Id3v1(_) => TagType::Id3v1,
            ReaderStrategy::Id3v2(_) => TagType::Id3v2,
            ReaderStrategy::Ape(_) => TagType::Ape,
        }
    }
    
    /// Get a meta entry using this strategy
    pub fn get_meta_entry(&self, path: &Path, entry: &MetaEntry) -> Result<Option<String>> {
        match self {
            ReaderStrategy::Id3v1(reader) => reader.get_meta_entry(path, entry),
            ReaderStrategy::Id3v2(reader) => reader.get_meta_entry(path, entry),
            ReaderStrategy::Ape(reader) => reader.get_meta_entry(path, entry),
        }
    }
}

/// Main tag reader class that uses the strategy pattern
pub struct TagReader {
    path: PathBuf,
    strategies: Vec<ReaderStrategy>,
}

impl TagReader {
    /// Create a new tag reader for the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        // Create strategies for each tag type in order of preference: ID3v2 > ID3v1 > APE
        let strategies = vec![
            ReaderStrategy::Id3v2(crate::id3::v2::tag::TagReader::new()),
            ReaderStrategy::Id3v1(crate::id3::v1::tag::TagReader::new()),
            ReaderStrategy::Ape(crate::ape::ApeReader::new()),
        ];
        
        let mut reader = Self {
            path,
            strategies,
        };
        
        reader.init()?;
        
        Ok(reader)
    }
    
    /// Initialize the reader
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Get a meta entry from the tag
    pub fn get_meta_entry(&self, entry: &MetaEntry) -> Result<String> {
        // Try each strategy in order until we find a value
        for strategy in &self.strategies {
            if let Ok(Some(value)) = strategy.get_meta_entry(&self.path, entry) {
                return Ok(value);
            }
        }

        // If no tag was found after trying all strategies, return an error.
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

/// Enum representing different tag writing strategies
#[derive(Debug)]
pub enum WriterStrategy {
    Id3v1(crate::id3::v1::tag::TagWriter),
    Id3v2(crate::id3::v2::tag::TagWriter),
    Ape(crate::ape::ApeWriter),
}

impl WriterStrategy {
    /// Get the tag type for this strategy
    pub fn tag_type(&self) -> TagType {
        match self {
            WriterStrategy::Id3v1(_) => TagType::Id3v1,
            WriterStrategy::Id3v2(_) => TagType::Id3v2,
            WriterStrategy::Ape(_) => TagType::Ape,
        }
    }
    
    /// Initialize the writer strategy
    pub fn init(&mut self, path: &Path) -> Result<()> {
        match self {
            WriterStrategy::Id3v1(writer) => writer.init(path),
            WriterStrategy::Id3v2(writer) => writer.init(path),
            WriterStrategy::Ape(writer) => writer.init(path),
        }
    }
    
    /// Set a meta entry using this strategy
    pub fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()> {
        match self {
            WriterStrategy::Id3v1(writer) => writer.set_meta_entry(entry, value),
            WriterStrategy::Id3v2(writer) => writer.set_meta_entry(entry, value),
            WriterStrategy::Ape(writer) => writer.set_meta_entry(entry, value),
        }
    }
    
    /// Save changes using this strategy
    pub fn save(&mut self) -> Result<()> {
        match self {
            WriterStrategy::Id3v1(writer) => writer.save(),
            WriterStrategy::Id3v2(writer) => writer.save(),
            WriterStrategy::Ape(writer) => writer.save(),
        }
    }
}

/// Main tag writer class that uses the strategy pattern
pub struct TagWriter {
    path: PathBuf,
    strategies: Vec<WriterStrategy>,
    preferred_tag_type: TagType,
}

/// Builder for TagWriter
pub struct TagWriterBuilder {
    path: PathBuf,
    strategies: Vec<WriterStrategy>,
    preferred_tag_type: TagType,
}

impl TagWriterBuilder {
    /// Create a new TagWriterBuilder
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            strategies: Vec::new(),
            preferred_tag_type: TagType::Id3v2, // Default to ID3v2
        }
    }
    
    /// Set the preferred tag type
    pub fn with_preferred_tag_type(mut self, tag_type: TagType) -> Self {
        self.preferred_tag_type = tag_type;
        self
    }
    
    /// Add a custom strategy
    pub fn with_strategy(mut self, strategy: WriterStrategy) -> Self {
        self.strategies.push(strategy);
        self
    }
    
    /// Build the TagWriter
    pub fn build(mut self) -> Result<TagWriter> {
        // If no strategies were added, add the default ones in order of preference: ID3v2 > ID3v1 > APE
        if self.strategies.is_empty() {
            self.strategies.push(WriterStrategy::Id3v2(crate::id3::v2::tag::TagWriter::new()));
            self.strategies.push(WriterStrategy::Id3v1(crate::id3::v1::tag::TagWriter::new()));
            self.strategies.push(WriterStrategy::Ape(crate::ape::ApeWriter::new()));
        }
        
        let mut writer = TagWriter {
            path: self.path,
            strategies: self.strategies,
            preferred_tag_type: self.preferred_tag_type,
        };
        
        writer.init()?;
        
        Ok(writer)
    }
}

impl TagWriter {
    /// Create a new tag writer for the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        TagWriterBuilder::new(path).build()
    }
    
    /// Initialize the writer
    fn init(&mut self) -> Result<()> {
        // Initialize each strategy
        for strategy in &mut self.strategies {
            let _ = strategy.init(&self.path);
        }
        
        Ok(())
    }
    
    /// Set the preferred tag type for writing
    pub fn set_preferred_tag_type(&mut self, tag_type: TagType) {
        self.preferred_tag_type = tag_type;
    }
    
    /// Set a meta entry in the tag
    pub fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()> {
        // First try to write to the preferred tag type
        for strategy in &mut self.strategies {
            if strategy.tag_type() == self.preferred_tag_type {
                let _ = strategy.set_meta_entry(&entry, value)?;
                let _ = strategy.save()?;
                return Ok(());
            }
        }
        
        // Try each strategy in order of preference
        for strategy in &mut self.strategies {
            // Try to set the entry using this strategy
            let _ = strategy.set_meta_entry(&entry, value)?;
            let _ = strategy.save()?;
            return Ok(());
        }
        
        // If that failed too, create a new tag of the preferred type
        for strategy in &mut self.strategies {
            if strategy.tag_type() == self.preferred_tag_type {
                let _ = strategy.set_meta_entry(&entry, value)?;
                let _ = strategy.save()?;
                return Ok(());
            }
        }
        
        Err(Error::Other("Failed to set meta entry".to_string()))
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
