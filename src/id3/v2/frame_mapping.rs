use std::collections::HashMap;
use std::sync::OnceLock;
use crate::meta_entry::MetaEntry;

/// Frame mapping for ID3v2.3 and ID3v2.4 (4-character frame IDs)
pub mod v3_v4 {
    use super::*;
    
    /// HashMap for ID3v2.3/v2.4 frame mappings
    static FRAME_MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    
    fn get_frame_map() -> &'static HashMap<&'static str, &'static str> {
        FRAME_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            m.insert("Title", "TIT2");
            m.insert("Artist", "TPE1");
            m.insert("Album", "TALB");
            m.insert("Year", "TYER");
            m.insert("Genre", "TCON");
            m.insert("Comment", "COMM");
            m.insert("Composer", "TCOM");
            m.insert("Track", "TRCK");
            m.insert("Date", "TDAT");
            m.insert("TextWriter", "TEXT");
            m.insert("AudioEncryption", "AENC");
            m.insert("Language", "TLAN");
            m.insert("Time", "TIME");
            m.insert("OriginalFilename", "TOFN");
            m.insert("FileType", "TFLT");
            m.insert("BandOrchestra", "TPE2");
            m
        })
    }
    
    pub fn get_frame_id(entry: &MetaEntry) -> Option<&'static str> {
        match entry {
            MetaEntry::Custom(_) => None, // Custom entries don't have predefined frame IDs
            _ => {
                let entry_name = format!("{}", entry);
                get_frame_map().get(entry_name.as_str()).copied()
            }
        }
    }
}

/// Frame mapping for ID3v2.0 (3-character frame IDs)
pub mod v2_0 {
    use super::*;
    
    /// HashMap for ID3v2.0 frame mappings
    static FRAME_MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    
    fn get_frame_map() -> &'static HashMap<&'static str, &'static str> {
        FRAME_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            m.insert("Title", "TIT");
            m.insert("Artist", "TP1");
            m.insert("Album", "TAL");
            m.insert("Date", "TDA");
            m.insert("Genre", "TCO");
            m.insert("TextWriter", "TXT");
            m.insert("AudioEncryption", "CRA");
            m.insert("Language", "TLA");
            m.insert("Time", "TIM");
            m.insert("Composer", "TCM");
            m.insert("FileType", "TFT");
            m.insert("BandOrchestra", "TP2");
            // Note: Some entries don't have equivalents in v2.0
            // Year, OriginalFilename, Track, Comment are not included
            m
        })
    }
    
    pub fn get_frame_id(entry: &MetaEntry) -> Option<&'static str> {
        match entry {
            MetaEntry::Custom(_) => None,
            _ => {
                let entry_name = format!("{}", entry);
                get_frame_map().get(entry_name.as_str()).copied()
            }
        }
    }
}
