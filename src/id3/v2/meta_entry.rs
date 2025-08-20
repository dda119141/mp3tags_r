use crate::meta_entry::MetaEntry;

/// ID3v2 supported meta entries
pub fn supported_entries() -> Vec<MetaEntry> {
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
        // Custom entries are also supported
    ]
}

/// Check if a MetaEntry is supported by ID3v2
pub fn is_supported(entry: &MetaEntry) -> bool {
    matches!(entry, 
        MetaEntry::Title |
        MetaEntry::Artist |
        MetaEntry::Album |
        MetaEntry::Year |
        MetaEntry::Genre |
        MetaEntry::Comment |
        MetaEntry::Composer |
        MetaEntry::Track |
        MetaEntry::Date |
        MetaEntry::TextWriter |
        MetaEntry::AudioEncryption |
        MetaEntry::Language |
        MetaEntry::Time |
        MetaEntry::OriginalFilename |
        MetaEntry::FileType |
        MetaEntry::BandOrchestra |
        MetaEntry::Custom(_)
    )
}
