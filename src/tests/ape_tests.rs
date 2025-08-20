use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::ape::{ApeReader, ApeWriter, common::{ApeTagHeader, ApeItem}};
use crate::tag::{TagReaderStrategy, TagWriterStrategy, TagType};
use crate::MetaEntry;
use crate::Result;

// Helper function to create a test MP3 file with an APE tag
fn create_test_file_with_ape_tag(path: &Path) -> Result<()> {
    // Create a minimal MP3 file with an APE tag
    let mut file = File::create(path)?;
    
    // Write some dummy MP3 data
    file.write_all(&[0xFF, 0xFB, 0x90, 0x44, 0x00])?;
    
    // Create an APE tag with some items
    let mut items = Vec::new();
    
    // Add title item
    let title_item = ApeItem::new_text("TITLE", "Test Title");
    items.push(title_item);
    
    // Add artist item
    let artist_item = ApeItem::new_text("ARTIST", "Test Artist");
    items.push(artist_item);
    
    // Add album item
    let album_item = ApeItem::new_text("ALBUM", "Test Album");
    items.push(album_item);
    
    // Add year item
    let year_item = ApeItem::new_text("YEAR", "2023");
    items.push(year_item);
    
    // Add genre item
    let genre_item = ApeItem::new_text("GENRE", "Test Genre");
    items.push(genre_item);
    
    // Add comment item
    let comment_item = ApeItem::new_text("COMMENT", "Test Comment");
    items.push(comment_item);
    
    // Add track item
    let track_item = ApeItem::new_text("TRACK", "1");
    items.push(track_item);
    
    // Calculate tag size
    let mut tag_size = 0;
    for item in &items {
        tag_size += item.size();
    }
    
    // Create tag header
    let header = ApeTagHeader::new(2000, tag_size as u32, items.len() as u32, true);
    
    // Write header
    file.write_all(&header.to_bytes())?;
    
    // Write items
    for item in &items {
        file.write_all(&item.to_bytes())?;
    }
    
    // Write footer (same as header but with footer flag)
    let footer = ApeTagHeader::new(2000, tag_size as u32, items.len() as u32, false);
    file.write_all(&footer.to_bytes())?;
    
    file.flush()?;
    
    Ok(())
}

// Helper function to clean up test files
fn cleanup_test_file(path: &Path) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_ape_reader_is_present() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_ape_reader_is_present.mp3");
    
    // Create a test file with an APE tag
    create_test_file_with_ape_tag(&path).unwrap();
    
    // Create an APE reader
    let reader = ApeReader::new();
    
    // Check if the tag is present
    let is_present = reader.is_present(&path).unwrap();
    assert!(is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_ape_reader_get_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_ape_reader_get_meta_entries.mp3");
    
    // Create a test file with an APE tag
    create_test_file_with_ape_tag(&path).unwrap();
    
    // Create an APE reader
    let reader = ApeReader::new();
    
    // Get meta entries
    let entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that all entries are present
    assert_eq!(entries.get(&MetaEntry::Title).unwrap(), "Test Title");
    assert_eq!(entries.get(&MetaEntry::Artist).unwrap(), "Test Artist");
    assert_eq!(entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(entries.get(&MetaEntry::Genre).unwrap(), "Test Genre");
    assert_eq!(entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(entries.get(&MetaEntry::Track).unwrap(), "1");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_ape_reader_get_meta_entry() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_ape_reader_get_meta_entry.mp3");
    
    // Create a test file with an APE tag
    create_test_file_with_ape_tag(&path).unwrap();
    
    // Create an APE reader
    let reader = ApeReader::new();
    
    // Get individual meta entries
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Title).unwrap(), Some("Test Title".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Artist).unwrap(), Some("Test Artist".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Album).unwrap(), Some("Test Album".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Year).unwrap(), Some("2023".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Genre).unwrap(), Some("Test Genre".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Comment).unwrap(), Some("Test Comment".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Track).unwrap(), Some("1".to_string()));
    
    // Get a non-existent meta entry
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Composer).unwrap(), None);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_ape_reader_tag_type() {
    // Create an APE reader
    let reader = ApeReader::new();
    
    // Check tag type
    assert_eq!(reader.tag_type(), TagType::Ape);
}

#[test]
fn test_ape_writer_is_present() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_ape_writer_is_present.mp3");
    
    // Create a test file with an APE tag
    create_test_file_with_ape_tag(&path).unwrap();
    
    // Create an APE writer
    let writer = ApeWriter::new();
    
    // Check if the tag is present
    let is_present = writer.is_present(&path).unwrap();
    assert!(is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_ape_writer_set_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_ape_writer_set_meta_entries.mp3");
    
    // Create a test file with an APE tag
    create_test_file_with_ape_tag(&path).unwrap();
    
    // Create an APE writer
    let writer = ApeWriter::new();
    
    // Create meta entries to set
    let mut entries = HashMap::new();
    entries.insert(MetaEntry::Title, "New Title".to_string());
    entries.insert(MetaEntry::Artist, "New Artist".to_string());
    entries.insert(MetaEntry::Composer, "New Composer".to_string());
    
    // Set meta entries
    writer.set_meta_entries(&path, &entries).unwrap();
    
    // Create an APE reader to verify the changes
    let reader = ApeReader::new();
    
    // Get meta entries
    let read_entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that the entries were updated
    assert_eq!(read_entries.get(&MetaEntry::Title).unwrap(), "New Title");
    assert_eq!(read_entries.get(&MetaEntry::Artist).unwrap(), "New Artist");
    assert_eq!(read_entries.get(&MetaEntry::Composer).unwrap(), "New Composer");
    
    // Check that other entries are still present
    assert_eq!(read_entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(read_entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(read_entries.get(&MetaEntry::Genre).unwrap(), "Test Genre");
    assert_eq!(read_entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(read_entries.get(&MetaEntry::Track).unwrap(), "1");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_ape_writer_remove_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_ape_writer_remove_meta_entries.mp3");
    
    // Create a test file with an APE tag
    create_test_file_with_ape_tag(&path).unwrap();
    
    // Create an APE writer
    let writer = ApeWriter::new();
    
    // Remove meta entries
    writer.remove_meta_entries(&path, &[MetaEntry::Title, MetaEntry::Artist]).unwrap();
    
    // Create an APE reader to verify the changes
    let reader = ApeReader::new();
    
    // Get meta entries
    let entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that the entries were removed
    assert!(!entries.contains_key(&MetaEntry::Title));
    assert!(!entries.contains_key(&MetaEntry::Artist));
    
    // Check that other entries are still present
    assert_eq!(entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(entries.get(&MetaEntry::Genre).unwrap(), "Test Genre");
    assert_eq!(entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(entries.get(&MetaEntry::Track).unwrap(), "1");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_ape_writer_remove_all_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_ape_writer_remove_all_meta_entries.mp3");
    
    // Create a test file with an APE tag
    create_test_file_with_ape_tag(&path).unwrap();
    
    // Create an APE writer
    let writer = ApeWriter::new();
    
    // Remove all meta entries
    writer.remove_all_meta_entries(&path).unwrap();
    
    // Create an APE reader to verify the changes
    let reader = ApeReader::new();
    
    // Check if the tag is present
    let is_present = reader.is_present(&path).unwrap();
    assert!(!is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_ape_writer_tag_type() {
    // Create an APE writer
    let writer = ApeWriter::new();
    
    // Check tag type
    assert_eq!(writer.tag_type(), TagType::Ape);
}

#[test]
fn test_ape_item_new_text() {
    // Create a text item
    let item = ApeItem::new_text("TITLE", "Test Title");
    
    // Check item properties
    assert_eq!(item.key, "TITLE");
    assert_eq!(item.value, "Test Title".as_bytes());
    assert_eq!(item.flags, 0); // Text item
}

#[test]
fn test_ape_item_new_binary() {
    // Create a binary item
    let item = ApeItem::new_binary("COVER", &[1, 2, 3, 4, 5]);
    
    // Check item properties
    assert_eq!(item.key, "COVER");
    assert_eq!(item.value, &[1, 2, 3, 4, 5]);
    assert_eq!(item.flags, 2); // Binary item
}

#[test]
fn test_ape_item_size() {
    // Create a text item
    let item = ApeItem::new_text("TITLE", "Test Title");
    
    // Check size
    // Size = 8 (value length) + 1 (null terminator) + 5 (key length) + 1 (null terminator) + 8 (flags + value length fields)
    assert_eq!(item.size(), 8 + 1 + 5 + 1 + 8);
}

#[test]
fn test_ape_tag_header() {
    // Create a tag header
    let header = ApeTagHeader::new(2000, 100, 5, true);
    
    // Check header properties
    assert_eq!(header.version, 2000);
    assert_eq!(header.tag_size, 100);
    assert_eq!(header.item_count, 5);
    assert!(header.is_header);
    
    // Convert to bytes and back
    let bytes = header.to_bytes();
    let header2 = ApeTagHeader::from_bytes(&bytes).unwrap();
    
    // Check that the header was correctly converted
    assert_eq!(header.version, header2.version);
    assert_eq!(header.tag_size, header2.tag_size);
    assert_eq!(header.item_count, header2.item_count);
    assert_eq!(header.is_header, header2.is_header);
}
