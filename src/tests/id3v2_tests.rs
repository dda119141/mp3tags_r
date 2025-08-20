use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::id3::{Id3v2Reader, Id3v2Writer};
use crate::tag::{TagReaderStrategy, TagWriterStrategy, TagType};
use crate::MetaEntry;
use crate::Result;

// Helper function to create a test MP3 file with an ID3v2 tag
fn create_test_file_with_id3v2_tag(path: &Path) -> Result<()> {
    // Create a minimal MP3 file with an ID3v2.3 tag
    let mut file = File::create(path)?;
    
    // ID3v2 header
    file.write_all(b"ID3")?; // Identifier
    file.write_all(&[3, 0])?; // Version 2.3.0
    file.write_all(&[0])?; // Flags
    
    // Calculate tag size (excluding header)
    // We'll add TIT2 (title), TPE1 (artist), TALB (album), TYER (year), TCON (genre), COMM (comment), TRCK (track)
    
    // TIT2 frame (title)
    let title_frame = create_text_frame(b"TIT2", "Test Title");
    
    // TPE1 frame (artist)
    let artist_frame = create_text_frame(b"TPE1", "Test Artist");
    
    // TALB frame (album)
    let album_frame = create_text_frame(b"TALB", "Test Album");
    
    // TYER frame (year)
    let year_frame = create_text_frame(b"TYER", "2023");
    
    // TCON frame (genre)
    let genre_frame = create_text_frame(b"TCON", "Pop");
    
    // COMM frame (comment)
    let comment_frame = create_comment_frame("Test Comment");
    
    // TRCK frame (track)
    let track_frame = create_text_frame(b"TRCK", "1");
    
    // Calculate total size
    let total_size = title_frame.len() + artist_frame.len() + album_frame.len() + 
                     year_frame.len() + genre_frame.len() + comment_frame.len() + 
                     track_frame.len();
    
    // Write syncsafe size (total_size as 4 syncsafe bytes)
    let syncsafe_size = syncsafe_integer(total_size as u32);
    file.write_all(&syncsafe_size)?;
    
    // Write frames
    file.write_all(&title_frame)?;
    file.write_all(&artist_frame)?;
    file.write_all(&album_frame)?;
    file.write_all(&year_frame)?;
    file.write_all(&genre_frame)?;
    file.write_all(&comment_frame)?;
    file.write_all(&track_frame)?;
    
    // Write some dummy MP3 data
    file.write_all(&[0xFF, 0xFB, 0x90, 0x44, 0x00])?;
    
    file.flush()?;
    
    Ok(())
}

// Helper function to create a text frame
fn create_text_frame(id: &[u8], text: &str) -> Vec<u8> {
    let mut frame = Vec::new();
    
    // Frame ID (4 bytes)
    frame.extend_from_slice(id);
    
    // Frame size (4 bytes, not syncsafe)
    // Size = encoding (1 byte) + text + null terminator
    let size = 1 + text.len() + 1;
    frame.extend_from_slice(&(size as u32).to_be_bytes());
    
    // Frame flags (2 bytes)
    frame.extend_from_slice(&[0, 0]);
    
    // Frame content
    frame.push(0); // UTF-8 encoding
    frame.extend_from_slice(text.as_bytes());
    frame.push(0); // Null terminator
    
    frame
}

// Helper function to create a comment frame
fn create_comment_frame(text: &str) -> Vec<u8> {
    let mut frame = Vec::new();
    
    // Frame ID (4 bytes)
    frame.extend_from_slice(b"COMM");
    
    // Frame size (4 bytes, not syncsafe)
    // Size = encoding (1 byte) + language (3 bytes) + description + null + text + null
    let size = 1 + 3 + 1 + text.len() + 1;
    frame.extend_from_slice(&(size as u32).to_be_bytes());
    
    // Frame flags (2 bytes)
    frame.extend_from_slice(&[0, 0]);
    
    // Frame content
    frame.push(0); // UTF-8 encoding
    frame.extend_from_slice(b"eng"); // Language
    frame.push(0); // Empty description
    frame.extend_from_slice(text.as_bytes());
    frame.push(0); // Null terminator
    
    frame
}

// Helper function to convert integer to syncsafe integer
fn syncsafe_integer(value: u32) -> [u8; 4] {
    let mut result = [0; 4];
    result[0] = ((value >> 21) & 0x7F) as u8;
    result[1] = ((value >> 14) & 0x7F) as u8;
    result[2] = ((value >> 7) & 0x7F) as u8;
    result[3] = (value & 0x7F) as u8;
    result
}

// Helper function to clean up test files
fn cleanup_test_file(path: &Path) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_id3v2_reader_is_present() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v2_reader_is_present.mp3");
    
    // Create a test file with an ID3v2 tag
    create_test_file_with_id3v2_tag(&path).unwrap();
    
    // Create an ID3v2 reader
    let reader = Id3v2Reader::new();
    
    // Check if the tag is present
    let is_present = reader.is_present(&path).unwrap();
    assert!(is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v2_reader_get_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v2_reader_get_meta_entries.mp3");
    
    // Create a test file with an ID3v2 tag
    create_test_file_with_id3v2_tag(&path).unwrap();
    
    // Create an ID3v2 reader
    let reader = Id3v2Reader::new();
    
    // Get meta entries
    let entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that all entries are present
    assert_eq!(entries.get(&MetaEntry::Title).unwrap(), "Test Title");
    assert_eq!(entries.get(&MetaEntry::Artist).unwrap(), "Test Artist");
    assert_eq!(entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(entries.get(&MetaEntry::Genre).unwrap(), "Pop");
    assert_eq!(entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(entries.get(&MetaEntry::Track).unwrap(), "1");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v2_reader_get_meta_entry() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v2_reader_get_meta_entry.mp3");
    
    // Create a test file with an ID3v2 tag
    create_test_file_with_id3v2_tag(&path).unwrap();
    
    // Create an ID3v2 reader
    let reader = Id3v2Reader::new();
    
    // Get individual meta entries
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Title).unwrap(), Some("Test Title".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Artist).unwrap(), Some("Test Artist".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Album).unwrap(), Some("Test Album".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Year).unwrap(), Some("2023".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Genre).unwrap(), Some("Pop".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Comment).unwrap(), Some("Test Comment".to_string()));
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Track).unwrap(), Some("1".to_string()));
    
    // Get a non-existent meta entry
    assert_eq!(reader.get_meta_entry(&path, MetaEntry::Composer).unwrap(), None);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v2_reader_tag_type() {
    // Create an ID3v2 reader
    let reader = Id3v2Reader::new();
    
    // Check tag type
    assert_eq!(reader.tag_type(), TagType::Id3v2);
}

#[test]
fn test_id3v2_writer_is_present() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v2_writer_is_present.mp3");
    
    // Create a test file with an ID3v2 tag
    create_test_file_with_id3v2_tag(&path).unwrap();
    
    // Create an ID3v2 writer
    let writer = Id3v2Writer::new();
    
    // Check if the tag is present
    let is_present = writer.is_present(&path).unwrap();
    assert!(is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v2_writer_set_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v2_writer_set_meta_entries.mp3");
    
    // Create a test file with an ID3v2 tag
    create_test_file_with_id3v2_tag(&path).unwrap();
    
    // Create an ID3v2 writer
    let writer = Id3v2Writer::new();
    
    // Create meta entries to set
    let mut entries = HashMap::new();
    entries.insert(MetaEntry::Title, "New Title".to_string());
    entries.insert(MetaEntry::Artist, "New Artist".to_string());
    entries.insert(MetaEntry::Composer, "New Composer".to_string());
    
    // Set meta entries
    writer.set_meta_entries(&path, &entries).unwrap();
    
    // Create an ID3v2 reader to verify the changes
    let reader = Id3v2Reader::new();
    
    // Get meta entries
    let read_entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that the entries were updated
    assert_eq!(read_entries.get(&MetaEntry::Title).unwrap(), "New Title");
    assert_eq!(read_entries.get(&MetaEntry::Artist).unwrap(), "New Artist");
    assert_eq!(read_entries.get(&MetaEntry::Composer).unwrap(), "New Composer");
    
    // Check that other entries are still present
    assert_eq!(read_entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(read_entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(read_entries.get(&MetaEntry::Genre).unwrap(), "Pop");
    assert_eq!(read_entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(read_entries.get(&MetaEntry::Track).unwrap(), "1");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v2_writer_remove_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v2_writer_remove_meta_entries.mp3");
    
    // Create a test file with an ID3v2 tag
    create_test_file_with_id3v2_tag(&path).unwrap();
    
    // Create an ID3v2 writer
    let writer = Id3v2Writer::new();
    
    // Remove meta entries
    writer.remove_meta_entries(&path, &[MetaEntry::Title, MetaEntry::Artist]).unwrap();
    
    // Create an ID3v2 reader to verify the changes
    let reader = Id3v2Reader::new();
    
    // Get meta entries
    let entries = reader.get_meta_entries(&path).unwrap();
    
    // Check that the entries were removed
    assert!(!entries.contains_key(&MetaEntry::Title));
    assert!(!entries.contains_key(&MetaEntry::Artist));
    
    // Check that other entries are still present
    assert_eq!(entries.get(&MetaEntry::Album).unwrap(), "Test Album");
    assert_eq!(entries.get(&MetaEntry::Year).unwrap(), "2023");
    assert_eq!(entries.get(&MetaEntry::Genre).unwrap(), "Pop");
    assert_eq!(entries.get(&MetaEntry::Comment).unwrap(), "Test Comment");
    assert_eq!(entries.get(&MetaEntry::Track).unwrap(), "1");
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v2_writer_remove_all_meta_entries() {
    // Create a temporary file path for testing
    let path = PathBuf::from("/tmp/test_mp3tags_r_id3v2_writer_remove_all_meta_entries.mp3");
    
    // Create a test file with an ID3v2 tag
    create_test_file_with_id3v2_tag(&path).unwrap();
    
    // Create an ID3v2 writer
    let writer = Id3v2Writer::new();
    
    // Remove all meta entries
    writer.remove_all_meta_entries(&path).unwrap();
    
    // Create an ID3v2 reader to verify the changes
    let reader = Id3v2Reader::new();
    
    // Check if the tag is present
    let is_present = reader.is_present(&path).unwrap();
    assert!(!is_present);
    
    // Clean up
    cleanup_test_file(&path);
}

#[test]
fn test_id3v2_writer_tag_type() {
    // Create an ID3v2 writer
    let writer = Id3v2Writer::new();
    
    // Check tag type
    assert_eq!(writer.tag_type(), TagType::Id3v2);
}
