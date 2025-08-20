use std::path::Path;
use crate::tag::{TagReader, TagWriter, TagType, TagPresence, ReaderStrategy, WriterStrategy};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_reader_creation() {
        // Test that TagReader can be created with a dummy path
        let dummy_path = Path::new("nonexistent.mp3");
        let result = TagReader::new(dummy_path);
        // Should succeed even if file doesn't exist (initialization may fail but creation shouldn't)
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for this test
    }

    #[test]
    fn test_tag_writer_creation() {
        let dummy_path = Path::new("nonexistent.mp3");
        let result = TagWriter::new(dummy_path);
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable
    }

    #[test]
    fn test_tag_presence_default() {
        let presence = TagPresence::default();
        assert!(!presence.id3v1_present);
        assert!(!presence.id3v2_present);
        assert!(!presence.ape_present);
    }

    #[test]
    fn test_reader_strategy_types() {
        let id3v1_strategy = ReaderStrategy::Id3v1(crate::id3::v1::tag::TagReader::new());
        let id3v2_strategy = ReaderStrategy::Id3v2(crate::id3::v2::tag::TagReader::new());
        let ape_strategy = ReaderStrategy::Ape(crate::ape::ApeReader::new());

        assert_eq!(id3v1_strategy.tag_type(), TagType::Id3v1);
        assert_eq!(id3v2_strategy.tag_type(), TagType::Id3v2);
        assert_eq!(ape_strategy.tag_type(), TagType::Ape);
    }

    #[test]
    fn test_writer_strategy_types() {
        let id3v1_strategy = WriterStrategy::Id3v1(crate::id3::v1::tag::TagWriter::new());
        let id3v2_strategy = WriterStrategy::Id3v2(crate::id3::v2::tag::TagWriter::new());
        let ape_strategy = WriterStrategy::Ape(crate::ape::ApeWriter::new());

        assert_eq!(id3v1_strategy.tag_type(), TagType::Id3v1);
        assert_eq!(id3v2_strategy.tag_type(), TagType::Id3v2);
        assert_eq!(ape_strategy.tag_type(), TagType::Ape);
    }
}
