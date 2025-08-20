use crate::{TagWriter, TagReader, MetaEntry};
use std::fs::copy;
use tempfile::tempdir;

#[test]
fn test_write_and_read_single_tag() {
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.mp3");
    
    // Copy the existing test file
    copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

    // Write a single tag
    let mut writer = TagWriter::new(&test_file).unwrap();
    writer.set_meta_entry(&MetaEntry::Title, "Test Title").unwrap();

    // Read it back
    let reader = TagReader::new(&test_file).unwrap();
    let title = reader.get_meta_entry(&MetaEntry::Title).unwrap();
    
    assert_eq!(title, "Test Title");
}

#[test]
fn test_write_multiple_tags() {
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.mp3");
    
    copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

    // Write multiple tags
    let mut writer = TagWriter::new(&test_file).unwrap();
    writer.set_meta_entry(&MetaEntry::Title, "Multi Title").unwrap();
    writer.set_meta_entry(&MetaEntry::Artist, "Multi Artist").unwrap();
    writer.set_meta_entry(&MetaEntry::Album, "Multi Album").unwrap();

    // Read them back
    let reader = TagReader::new(&test_file).unwrap();
    
    assert_eq!(reader.get_meta_entry(&MetaEntry::Title).unwrap(), "Multi Title");
    assert_eq!(reader.get_meta_entry(&MetaEntry::Artist).unwrap(), "Multi Artist");
    assert_eq!(reader.get_meta_entry(&MetaEntry::Album).unwrap(), "Multi Album");
}

#[test]
fn test_tag_preservation() {
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.mp3");
    
    copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

    // Write first tag
    let mut writer = TagWriter::new(&test_file).unwrap();
    writer.set_meta_entry(&MetaEntry::Title, "Original Title").unwrap();

    // Write second tag (should preserve first)
    let mut writer = TagWriter::new(&test_file).unwrap();
    writer.set_meta_entry(&MetaEntry::Artist, "New Artist").unwrap();

    // Verify both tags exist
    let reader = TagReader::new(&test_file).unwrap();
    assert_eq!(reader.get_meta_entry(&MetaEntry::Title).unwrap(), "Original Title");
    assert_eq!(reader.get_meta_entry(&MetaEntry::Artist).unwrap(), "New Artist");
}

#[test]
fn test_unicode_content() {
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.mp3");
    
    copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

    let unicode_title = "Test 测试 🎵";
    
    // Write unicode tag
    let mut writer = TagWriter::new(&test_file).unwrap();
    writer.set_meta_entry(&MetaEntry::Title, unicode_title).unwrap();

    // Read it back
    let reader = TagReader::new(&test_file).unwrap();
    let title = reader.get_meta_entry(&MetaEntry::Title).unwrap();
    
    assert_eq!(title, unicode_title);
}

#[test]
fn test_empty_tag_values() {
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.mp3");
    
    copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

    // Write empty tag
    let mut writer = TagWriter::new(&test_file).unwrap();
    writer.set_meta_entry(&MetaEntry::Title, "").unwrap();

    // Read it back
    let reader = TagReader::new(&test_file).unwrap();
    let title = reader.get_meta_entry(&MetaEntry::Title).unwrap();
    
    assert_eq!(title, "");
}
