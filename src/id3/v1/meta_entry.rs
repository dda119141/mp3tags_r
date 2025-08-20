use crate::meta_entry::MetaEntry;

/// ID3v1 supported meta entries
pub fn supported_entries() -> Vec<MetaEntry> {
    vec![
        MetaEntry::Title,
        MetaEntry::Artist,
        MetaEntry::Album,
        MetaEntry::Year,
        MetaEntry::Comment,
        // Note: ID3v1 doesn't support the extended entries like Date, TextWriter, etc.
    ]
}

/// Check if a MetaEntry is supported by ID3v1
pub fn is_supported(entry: &MetaEntry) -> bool {
    matches!(entry, 
        MetaEntry::Title |
        MetaEntry::Artist |
        MetaEntry::Album |
        MetaEntry::Year |
        MetaEntry::Comment
    )
}
