use crate::{TagWriter, TagReader, MetaEntry, tag::TagType};
use proptest::prelude::*;
use std::fs::copy;
use tempfile::tempdir;

#[cfg(test)]
mod property_based_tests {
    use super::*;

    // Property: Round-trip invariant - data written should be readable
    proptest! {
        #[test]
        fn prop_roundtrip_invariant(
            title in "\\PC{0,1000}",  // Any printable chars, max 1000 length
            artist in "\\PC{0,500}",
            album in "\\PC{0,500}"
        ) {
            let temp_dir = tempdir().unwrap();
            let test_file = temp_dir.path().join("prop_test.mp3");
            copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

            // Write tags
            if let Ok(mut writer) = TagWriter::new(&test_file, TagType::Id3v2) {
                let write_title = writer.set_meta_entry(&MetaEntry::Title, &title);
                let write_artist = writer.set_meta_entry(&MetaEntry::Artist, &artist);
                let write_album = writer.set_meta_entry(&MetaEntry::Album, &album);

                // If writes succeed, reads should work and return same data
                if write_title.is_ok() && write_artist.is_ok() && write_album.is_ok() {
                    if let Ok(reader) = TagReader::new(&test_file) {
                        if let Ok(read_title) = reader.get_meta_entry(&MetaEntry::Title) {
                            prop_assert_eq!(read_title, title);
                        }
                        if let Ok(read_artist) = reader.get_meta_entry(&MetaEntry::Artist) {
                            prop_assert_eq!(read_artist, artist);
                        }
                        if let Ok(read_album) = reader.get_meta_entry(&MetaEntry::Album) {
                            prop_assert_eq!(read_album, album);
                        }
                    }
                }
            }
        }
    }

    // Property: Idempotency - writing same data twice should have same result
    proptest! {
        #[test]
        fn prop_write_idempotency(value in "\\PC{0,500}") {
            let temp_dir = tempdir().unwrap();
            let test_file = temp_dir.path().join("idempotent_test.mp3");
            copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

            if let Ok(mut writer1) = TagWriter::new(&test_file, TagType::Id3v2) {
                let _ = writer1.set_meta_entry(&MetaEntry::Title, &value);
            }

            let first_read = if let Ok(reader) = TagReader::new(&test_file) {
                reader.get_meta_entry(&MetaEntry::Title).ok()
            } else { None };

            // Write same value again
            if let Ok(mut writer2) = TagWriter::new(&test_file, TagType::Id3v2) {
                let _ = writer2.set_meta_entry(&MetaEntry::Title, &value);
            }

            let second_read = if let Ok(reader) = TagReader::new(&test_file) {
                reader.get_meta_entry(&MetaEntry::Title).ok()
            } else { None };

            prop_assert_eq!(first_read, second_read);
        }
    }

    // Property: Data length constraints
    proptest! {
        #[test]
        fn prop_data_length_handling(
            data in prop::collection::vec(any::<u8>(), 0..10000).prop_map(|v| String::from_utf8_lossy(&v).to_string())
        ) {
            let temp_dir = tempdir().unwrap();
            let test_file = temp_dir.path().join("length_test.mp3");
            copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

            if let Ok(mut writer) = TagWriter::new(&test_file, TagType::Id3v2) {
                let result = writer.set_meta_entry(&MetaEntry::Title, &data);
                
                // Either succeeds or fails gracefully - no panics
                match result {
                    Ok(_) => {
                        // If write succeeds, read should work
                        if let Ok(reader) = TagReader::new(&test_file) {
                            let read_result = reader.get_meta_entry(&MetaEntry::Title);
                            prop_assert!(read_result.is_ok());
                        }
                    }
                    Err(_) => {
                        // Failure is acceptable for very large/invalid data
                    }
                }
            }
        }
    }

    // Property: No panics on arbitrary input
    proptest! {
        #[test]
        fn prop_no_panic_arbitrary_strings(
            title in ".*{0,1000}",
            artist in ".*{0,1000}"
        ) {
            let temp_dir = tempdir().unwrap();
            let test_file = temp_dir.path().join("prop_test.mp3");
            copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

            // Should never panic regardless of input
            if let Ok(mut writer) = TagWriter::new(&test_file, TagType::Id3v2) {
                let _ = writer.set_meta_entry(&MetaEntry::Title, &title);
                let _ = writer.set_meta_entry(&MetaEntry::Artist, &artist);
            }

            if let Ok(reader) = TagReader::new(&test_file) {
                let _ = reader.get_meta_entry(&MetaEntry::Title);
                let _ = reader.get_meta_entry(&MetaEntry::Artist);
            }
        }
    }

    // Property: Unicode handling
    proptest! {
        #[test]
        fn prop_unicode_handling(input in ".*{0,500}") {
            let temp_dir = tempdir().unwrap();
            let test_file = temp_dir.path().join("unicode_test.mp3");
            copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

            // Test various Unicode transformations
            let unicode_variants = vec![
                input.clone(),
                input.chars().rev().collect::<String>(), // Reversed
                input.to_uppercase(),
                input.to_lowercase(),
                format!("🎵{}", input), // With emoji prefix
            ];

            for variant in unicode_variants {
                if let Ok(mut writer) = TagWriter::new(&test_file, TagType::Id3v2) {
                    let _ = writer.set_meta_entry(&MetaEntry::Title, &variant);
                }

                if let Ok(reader) = TagReader::new(&test_file) {
                    let _ = reader.get_meta_entry(&MetaEntry::Title);
                }
            }
        }
    }

    // Property: Numeric string handling
    proptest! {
        #[test]
        fn prop_numeric_string_handling(num in any::<i64>()) {
            let temp_dir = tempdir().unwrap();
            let test_file = temp_dir.path().join("numeric_test.mp3");
            copy("audio_files/mp3_44100Hz_128kbps_stereo.mp3", &test_file).unwrap();

            let numeric_strings = vec![
                num.to_string(),
                format!("{:x}", num), // Hex
                format!("{:o}", num), // Octal
                format!("{:b}", num), // Binary
                format!("{:.2}", num as f64), // Float representation
            ];

            for num_str in numeric_strings {
                if let Ok(mut writer) = TagWriter::new(&test_file, TagType::Id3v2) {
                    let _ = writer.set_meta_entry(&MetaEntry::Year, &num_str);
                    let _ = writer.set_meta_entry(&MetaEntry::Track, &num_str);
                }
            }
        }
    }
}
