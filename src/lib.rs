//! mp3tags_r - A Rust library for reading and writing MP3 tags (ID3 and APE)
//!
//! This library provides functionality to read and write ID3 and APE tags in MP3 files.
//! It uses template and strategy patterns to provide a clean and extensible API.

pub mod error;
pub mod meta_entry;
pub mod util;
pub mod tag;
pub mod id3;
pub mod ape;
pub mod validation;

pub use error::{Error, Result};
pub use meta_entry::MetaEntry;
pub use tag::{TagReader, TagWriter, TagType, TagPresence};

// Re-export common tag operations for convenience
pub use tag::{
    get_title,
    get_artist,
    get_album,
    get_year,
    get_genre,
    get_comment,
    get_composer,
    get_all_meta_entries,
};

#[cfg(test)]
mod tests;
