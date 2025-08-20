use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::id3::{Id3v1Reader, Id3v1Writer};
use crate::tag::{TagReaderStrategy, TagWriterStrategy, TagType};
use crate::MetaEntry;
use crate::Result;

// Helper function to create a test MP3 file with an ID3v1 tag
fn create_test_file_with_id3v1_tag(path: &Path) -> Result<()> {
    // Create a minimal MP3 file with an ID3v1 tag
    let mut file = File::create(path)?;
    
    // Write some dummy MP3 data
    file.write_all(&[0xFF, 0xFB, 0x90, 0x44, 0x00])?;
    
    // Seek to position for ID3v1 tag (end of file - 128 bytes)
    file.seek(SeekFrom::End(-128))?;
    
    // Write ID3v1 tag
    file.write_all(b"TAG")?; // ID3v1 identifier
    file.write_all(b"Test Title".to_string().as_bytes())?; // Title (30 bytes)
    file.write_all(&[0; 30 - b"Test Title".len()])?;
    
    file.write_all(b"Test Artist".to_string().as_bytes())?; // Artist (30 bytes)
    file.write_all(&[0; 30 - b"Test Artist".len()])?;
    
    file.write_all(b"Test Album".to_string().as_bytes())?; // Album (30 bytes)
    file.write_all(&[0; 30 - b"Test Album".len()])?;
    
    file.write_all(b"2023")?; // Year (4 bytes)
    
    file.write_all(b"Test Comment")?; // Comment (28 bytes for ID3v1.1)
    file.write_all(&[0; 28 - b"Test Comment".len()])?;
    
    file.write_all(&[0])?; // Zero byte for ID3v1.1
    file.write_all(&[1])?; // Track number
    
    file.write_all(&[1])?; // Genre (Pop)
    
    file.flush()?;
    
    Ok(())
}

// Helper function to clean up test files
fn cleanup_test_file(path: &Path) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_id3v1_reader_is_present() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v1_reader_is_present.mp3");
    
    // Create a test file with an ID3v1 tag
    create_test_file_with_id3v1_tag(&path).unwrap();
    
    // Create an ID3v1 reader
    let reader = Id3v1Reader::new();
    
    // Check if the tag is present
    let is_present = reader.is_present(&path).unwrap();
    assert!(is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v1_reader_get_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v1_reader_get_meta_entries.mp3");
    
    // Create a test file with an ID3v1 tag
    create_test_file_with_id3v1_tag(&path).unwrap();
    
    // Create an ID3v1 reader
    let reader = Id3v1Reader::new();
    
    // Get meta entries
    let entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that all entries are present
    assert_eq!(entries.get(&MetaEntry::Title).unwrap(), "Test Title");
    assert_eq!(entries.get(&MetaEntry::Artist).unwrap(), "Test Artist");
    assert_eq!(entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(entries.get(&MetaEntry::Track).unwrap(), "1");
    assert_eq!(entries.get(&MetaEntry::Genre).unwrap(), "Pop");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v1_reader_get_meta_entry() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v1_reader_get_meta_entry.mp3");
    
    // Create a test file with an ID3v1 tag
    create_test_file_with_id3v1_tag(&path).unwrap();
    
    // Create an ID3v1 reader
    let reader = Id3v1Reader::new();
    
    // Get individual meta entries
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Title).unwrap(), Some("Test Title".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Artist).unwrap(), Some("Test Artist".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Album).unwrap(), Some("Test Album".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Year).unwrap(), Some("2023".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Comment).unwrap(), Some("Test Comment".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Track).unwrap(), Some("1".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Genre).unwrap(), Some("Pop".to_string()));
    
    // Get a non-existent meta entry
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Composer).unwrap(), None);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v1_reader_tag_type() {
    // Create an ID3v1 reader
    let reader = Id3v1Reader::new();
    
    // Check tag type
    assert_eq!(reader.tag_type(), TagType::Id3v1);
}

#[test]
fn test_id3v1_writer_is_present() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v1_writer_is_present.mp3");
    
    // Create a test file with an ID3v1 tag
    create_test_file_with_id3v1_tag(&path).unwrap();
    
    // Create an ID3v1 writer
    let writer = Id3v1Writer::new();
    
    // Check if the tag is present
    let is_present = writer.is_present(&path).unwrap();
    assert!(is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v1_writer_set_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v1_writer_set_meta_entries.mp3");
    
    // Create a test file with an ID3v1 tag
    create_test_file_with_id3v1_tag(&path).unwrap();
    
    // Create an ID3v1 writer
    let writer = Id3v1Writer::new();
    
    // Create meta entries to set
    let mut entries = HashMap::new();
    entries.insert(MetaEntry::Title, "New Title".to_string());
    entries.insert(MetaEntry::Artist, "New Artist".to_string());
    entries.insert(MetaEntry::Track, "5".to_string());
    
    // Set meta entries
    writer.set_meta_entries(&path, &entries).unwrap();
    
    // Create an ID3v1 reader to verify the changes
    let reader = Id3v1Reader::new();
    
    // Get meta entries
    let read_entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that the entries were updated
    assert_eq!(read_entries.get(&MetaEntry::Title).unwrap(), "New Title");
    assert_eq!(read_entries.get(&MetaEntry::Artist).unwrap(), "New Artist");
    assert_eq!(read_entries.get(&MetaEntry::Track).unwrap(), "5");
    
    // Check that other entries are still present
    assert_eq!(read_entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(read_entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(read_entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(read_entries.get(&MetaEntry::Genre).unwrap(), "Pop");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v1_writer_remove_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v1_writer_remove_meta_entries.mp3");
    
    // Create a test file with an ID3v1 tag
    create_test_file_with_id3v1_tag(&path).unwrap();
    
    // Create an ID3v1 writer
    let writer = Id3v1Writer::new();
    
    // Remove meta entries
    writer.remove_meta_entries(&path, &[MetaEntry::Title, MetaEntry::Artist]).unwrap();
    
    // Create an ID3v1 reader to verify the changes
    let reader = Id3v1Reader::new();
    
    // Get meta entries
    let entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that the entries were removed (set to empty strings in ID3v1)
    assert_eq!(entries.get(&MetaEntry::Title).unwrap(), "");
    assert_eq!(entries.get(&MetaEntry::Artist).unwrap(), "");
    
    // Check that other entries are still present
    assert_eq!(entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(entries.get(&MetaEntry::Track).unwrap(), "1");
    assert_eq!(entries.get(&MetaEntry::Genre).unwrap(), "Pop");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v1_writer_remove_all_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v1_writer_remove_all_meta_entries.mp3");
    
    // Create a test file with an ID3v1 tag
    create_test_file_with_id3v1_tag(&path).unwrap();
    
    // Create an ID3v1 writer
    let writer = Id3v1Writer::new();
    
    // Remove all meta entries
    writer.remove_all_meta_entries(&path).unwrap();
    
    // Create an ID3v1 reader to verify the changes
    let reader = Id3v1Reader::new();
    
    // Check if the tag is present
    let is_present = reader.is_present(&path).unwrap();
    assert!(!is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v1_writer_tag_type() {
    // Create an ID3v1 writer
    let writer = Id3v1Writer::new();
    
    // Check tag type
    assert_eq!(writer.tag_type(), TagType::Id3v1);
}
