//! Metadata entry definitions for audio tag manipulation.
//!
//! This module provides the core `MetaEntry` enum that represents different types
//! of metadata fields that can be stored in audio tags.

use std::fmt;

/// Represents different types of metadata entries that can be stored in audio tags.
/// 
/// Not all tag formats support all entry types:
/// - ID3v1: Only supports core entries (Title, Artist, Album, Year, Comment)
/// - ID3v2: Supports all entries with version-specific frame mappings  
/// - APE: Supports all entries with custom key names
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetaEntry {
    // Core entries (supported by most formats)
    Title,
    Artist,
    Album,
    Year,
    Genre,
    Comment,
    
    // Extended entries (ID3v2 and APE)
    Composer,
    Track,
    Date,
    TextWriter,
    AudioEncryption,
    Language,
    Time,
    OriginalFilename,
    FileType,
    BandOrchestra,
    
    /// Custom entry with user-defined key
    Custom(String),
}

impl fmt::Display for MetaEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title => write!(f, "Title"),
            Self::Artist => write!(f, "Artist"),
            Self::Album => write!(f, "Album"),
            Self::Year => write!(f, "Year"),
            Self::Genre => write!(f, "Genre"),
            Self::Comment => write!(f, "Comment"),
            Self::Composer => write!(f, "Composer"),
            Self::Track => write!(f, "Track"),
            Self::Date => write!(f, "Date"),
            Self::TextWriter => write!(f, "TextWriter"),
            Self::AudioEncryption => write!(f, "AudioEncryption"),
            Self::Language => write!(f, "Language"),
            Self::Time => write!(f, "Time"),
            Self::OriginalFilename => write!(f, "OriginalFilename"),
            Self::FileType => write!(f, "FileType"),
            Self::BandOrchestra => write!(f, "BandOrchestra"),
            Self::Custom(key) => write!(f, "{}", key),
        }
    }
}

/// Returns all standard meta entries (excludes Custom).
pub fn all_standard_entries() -> Vec<MetaEntry> {
    vec![
        MetaEntry::Title,
        MetaEntry::Artist,
        MetaEntry::Album,
        MetaEntry::Year,
        MetaEntry::Genre,
        MetaEntry::Comment,
        MetaEntry::Composer,
        MetaEntry::Track,
        MetaEntry::Date,
        MetaEntry::TextWriter,
        MetaEntry::AudioEncryption,
        MetaEntry::Language,
        MetaEntry::Time,
        MetaEntry::OriginalFilename,
        MetaEntry::FileType,
        MetaEntry::BandOrchestra,
    ]
}
