use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::tag::{TagReader, TagWriter, TagReaderStrategy, TagWriterStrategy, TagType, TagPresence};
use crate::MetaEntry;
use crate::Result;

// Mock tag reader strategy for testing
struct MockTagReaderStrategy {
    tag_type: TagType,
    is_present_result: bool,
    meta_entries: HashMap<MetaEntry, String>,
}

impl MockTagReaderStrategy {
    fn new(tag_type: TagType, is_present: bool) -> Self {
        Self {
            tag_type,
            is_present_result: is_present,
            meta_entries: HashMap::new(),
        }
    }
    
    fn with_entry(mut self, entry: MetaEntry, value: &str) -> Self {
        self.meta_entries.insert(entry, value.to_string());
        self
    }
}

impl TagReaderStrategy for MockTagReaderStrategy {
    fn is_present<P: AsRef<Path>>(&self, _path: P) -> Result<bool> {
        Ok(self.is_present_result)
    }
    
    fn get_meta_entries<P: AsRef<Path>>(&self, _path: P) -> Result<HashMap<MetaEntry, String>> {
        Ok(self.meta_entries.clone())
    }
    
    fn get_meta_entry<P: AsRef<Path>>(&self, _path: P, entry: MetaEntry) -> Result<Option<String>> {
        Ok(self.meta_entries.get(&entry).cloned())
    }
    
    fn tag_type(&self) -> TagType {
        self.tag_type
    }
}

// Mock tag writer strategy for testing
struct MockTagWriterStrategy {
    tag_type: TagType,
    is_present_result: bool,
    written_entries: HashMap<MetaEntry, String>,
    removed_entries: Vec<MetaEntry>,
    all_removed: bool,
}

impl MockTagWriterStrategy {
    fn new(tag_type: TagType, is_present: bool) -> Self {
        Self {
            tag_type,
            is_present_result: is_present,
            written_entries: HashMap::new(),
            removed_entries: Vec::new(),
            all_removed: false,
        }
    }
}

impl TagWriterStrategy for MockTagWriterStrategy {
    fn is_present<P: AsRef<Path>>(&self, _path: P) -> Result<bool> {
        Ok(self.is_present_result)
    }
    
    fn set_meta_entries<P: AsRef<Path>>(&self, _path: P, entries: &HashMap<MetaEntry, String>) -> Result<()> {
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        mut_self.written_entries.extend(entries.clone());
        Ok(())
    }
    
    fn remove_meta_entries<P: AsRef<Path>>(&self, _path: P, entries: &[MetaEntry]) -> Result<()> {
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        mut_self.removed_entries.extend(entries.iter().cloned());
        Ok(())
    }
    
    fn remove_all_meta_entries<P: AsRef<Path>>(&self, _path: P) -> Result<()> {
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        mut_self.all_removed = true;
        Ok(())
    }
    
    fn tag_type(&self) -> TagType {
        self.tag_type
    }
}

#[test]
fn test_tag_reader_get_meta_entry_preference_order() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_tag_reader.mp3");
    
    // Create a tag reader
    let mut reader = TagReader::new(&path).unwrap();
    
    // Add mock strategies with different tag types
    let id3v1_strategy = MockTagReaderStrategy::new(TagType::Id3v1, true)
        .with_entry(MetaEntry::Title, "ID3v1 Title")
        .with_entry(MetaEntry::Artist, "ID3v1 Artist");
    
    let id3v2_strategy = MockTagReaderStrategy::new(TagType::Id3v2, true)
        .with_entry(MetaEntry::Title, "ID3v2 Title")
        .with_entry(MetaEntry::Artist, "ID3v2 Artist")
        .with_entry(MetaEntry::Album, "ID3v2 Album");
    
    let ape_strategy = MockTagReaderStrategy::new(TagType::Ape, true)
        .with_entry(MetaEntry::Title, "APE Title")
        .with_entry(MetaEntry::Genre, "APE Genre");
    
    reader.add_strategy(id3v1_strategy).unwrap();
    reader.add_strategy(id3v2_strategy).unwrap();
    reader.add_strategy(ape_strategy).unwrap();
    
    // Test preference order: APE > ID3v2 > ID3v1
    assert_eq!(reader.get_meta_entry(MetaEntry::Title).unwrap(), "APE Title");
    assert_eq!(reader.get_meta_entry(MetaEntry::Artist).unwrap(), "ID3v2 Artist");
    assert_eq!(reader.get_meta_entry(MetaEntry::Album).unwrap(), "ID3v2 Album");
    assert_eq!(reader.get_meta_entry(MetaEntry::Genre).unwrap(), "APE Genre");
    
    // Test tag presence
    let presence = reader.tag_presence();
    assert!(presence.id3v1_present);
    assert!(presence.id3v2_present);
    assert!(presence.ape_present);
    assert!(presence.has_any_tag());
}

#[test]
fn test_tag_reader_get_all_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_tag_reader.mp3");
    
    // Create a tag reader
    let mut reader = TagReader::new(&path).unwrap();
    
    // Add mock strategies with different tag types
    let id3v1_strategy = MockTagReaderStrategy::new(TagType::Id3v1, true)
        .with_entry(MetaEntry::Title, "ID3v1 Title")
        .with_entry(MetaEntry::Artist, "ID3v1 Artist");
    
    let id3v2_strategy = MockTagReaderStrategy::new(TagType::Id3v2, true)
        .with_entry(MetaEntry::Title, "ID3v2 Title")
        .with_entry(MetaEntry::Artist, "ID3v2 Artist")
        .with_entry(MetaEntry::Album, "ID3v2 Album");
    
    let ape_strategy = MockTagReaderStrategy::new(TagType::Ape, true)
        .with_entry(MetaEntry::Title, "APE Title")
        .with_entry(MetaEntry::Genre, "APE Genre");
    
    reader.add_strategy(id3v1_strategy).unwrap();
    reader.add_strategy(id3v2_strategy).unwrap();
    reader.add_strategy(ape_strategy).unwrap();
    
    // Get all meta entries
    let entries = reader.get_all_meta_entries();
    
    // Check that entries from all strategies are present
    // APE entries should override ID3v2 entries, which should override ID3v1 entries
    assert_eq!(entries.get(&MetaEntry::Title).unwrap(), "APE Title");
    assert_eq!(entries.get(&MetaEntry::Artist).unwrap(), "ID3v2 Artist");
    assert_eq!(entries.get(&MetaEntry::Album).unwrap(), "ID3v2 Album");
    assert_eq!(entries.get(&MetaEntry::Genre).unwrap(), "APE Genre");
}

#[test]
fn test_tag_writer_set_meta_entry() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_tag_writer.mp3");
    
    // Create a tag writer
    let mut writer = TagWriter::new(&path).unwrap();
    
    // Add mock strategies with different tag types
    let id3v1_strategy = MockTagWriterStrategy::new(TagType::Id3v1, true);
    let id3v2_strategy = MockTagWriterStrategy::new(TagType::Id3v2, true);
    let ape_strategy = MockTagWriterStrategy::new(TagType::Ape, true);
    
    writer.add_strategy(id3v1_strategy).unwrap();
    writer.add_strategy(id3v2_strategy).unwrap();
    writer.add_strategy(ape_strategy).unwrap();
    
    // Set preferred tag type to ID3v2 (default)
    writer.set_preferred_tag_type(TagType::Id3v2);
    
    // Set a meta entry
    writer.set_meta_entry(&MetaEntry::Title, "Test Title").unwrap();
    
    // Check that the entry was written to the preferred strategy
    let id3v2_strategy = writer.strategies.iter()
        .find(|s| s.tag_type() == TagType::Id3v2)
        .unwrap();
    let id3v2_strategy = id3v2_strategy.downcast_ref::<MockTagWriterStrategy>().unwrap();
    assert_eq!(id3v2_strategy.written_entries.get(&MetaEntry::Title).unwrap(), "Test Title");
    
    // Change preferred tag type to APE
    writer.set_preferred_tag_type(TagType::Ape);
    
    // Set another meta entry
    writer.set_meta_entry(&MetaEntry::Artist, "Test Artist").unwrap();
    
    // Check that the entry was written to the new preferred strategy
    let ape_strategy = writer.strategies.iter()
        .find(|s| s.tag_type() == TagType::Ape)
        .unwrap();
    let ape_strategy = ape_strategy.downcast_ref::<MockTagWriterStrategy>().unwrap();
    assert_eq!(ape_strategy.written_entries.get(&MetaEntry::Artist).unwrap(), "Test Artist");
}

#[test]
fn test_tag_writer_remove_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_tag_writer.mp3");
    
    // Create a tag writer
    let mut writer = TagWriter::new(&path).unwrap();
    
    // Add mock strategies with different tag types
    let id3v1_strategy = MockTagWriterStrategy::new(TagType::Id3v1, true);
    let id3v2_strategy = MockTagWriterStrategy::new(TagType::Id3v2, true);
    let ape_strategy = MockTagWriterStrategy::new(TagType::Ape, true);
    
    writer.add_strategy(id3v1_strategy).unwrap();
    writer.add_strategy(id3v2_strategy).unwrap();
    writer.add_strategy(ape_strategy).unwrap();
    
    // Remove a meta entry
    writer.remove_meta_entry(MetaEntry::Title).unwrap();
    
    // Check that the entry was removed from all strategies
    for strategy in &writer.strategies {
        let strategy = strategy.downcast_ref::<MockTagWriterStrategy>().unwrap();
        assert!(strategy.removed_entries.contains(&MetaEntry::Title));
    }
    
    // Remove multiple meta entries
    writer.remove_meta_entries(&[MetaEntry::Artist, MetaEntry::Album]).unwrap();
    
    // Check that the entries were removed from all strategies
    for strategy in &writer.strategies {
        let strategy = strategy.downcast_ref::<MockTagWriterStrategy>().unwrap();
        assert!(strategy.removed_entries.contains(&MetaEntry::Artist));
        assert!(strategy.removed_entries.contains(&MetaEntry::Album));
    }
}

#[test]
fn test_tag_writer_remove_all_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_tag_writer.mp3");
    
    // Create a tag writer
    let mut writer = TagWriter::new(&path).unwrap();
    
    // Add mock strategies with different tag types
    let id3v1_strategy = MockTagWriterStrategy::new(TagType::Id3v1, true);
    let id3v2_strategy = MockTagWriterStrategy::new(TagType::Id3v2, true);
    let ape_strategy = MockTagWriterStrategy::new(TagType::Ape, true);
    
    writer.add_strategy(id3v1_strategy).unwrap();
    writer.add_strategy(id3v2_strategy).unwrap();
    writer.add_strategy(ape_strategy).unwrap();
    
    // Remove all meta entries
    writer.remove_all_meta_entries().unwrap();
    
    // Check that all entries were removed from all strategies
    for strategy in &writer.strategies {
        let strategy = strategy.downcast_ref::<MockTagWriterStrategy>().unwrap();
        assert!(strategy.all_removed);
    }
}
