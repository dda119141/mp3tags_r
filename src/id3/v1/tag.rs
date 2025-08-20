use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

use crate::error::{Error, Result};
use crate::meta_entry::MetaEntry;
use crate::tag::{TagType, TagReaderStrategy, TagWriterStrategy};
use crate::id3::constants::{ID3V1_TAG_SIZE, ID3V1_IDENTIFIER};

// ID3v1 field sizes
const TITLE_SIZE: usize = 30;
const ARTIST_SIZE: usize = 30;
const ALBUM_SIZE: usize = 30;
const YEAR_SIZE: usize = 4;
const COMMENT_SIZE: usize = 30;
const GENRE_SIZE: usize = 1;
const IDENTIFIER_SIZE: usize = 3;

// ID3v1 field offsets
const IDENTIFIER_OFFSET: usize = 0;
const TITLE_OFFSET: usize = 3;
const ARTIST_OFFSET: usize = 33;
const ALBUM_OFFSET: usize = 63;
const YEAR_OFFSET: usize = 93;
const COMMENT_OFFSET: usize = 97;
const GENRE_OFFSET: usize = 127;

pub fn has_id3v1_tag(path: &std::path::Path) -> crate::Result<bool> {
    use std::io::{Read, Seek, SeekFrom};
    let mut file = std::fs::File::open(path)?;
    file.seek(SeekFrom::End(-(ID3V1_TAG_SIZE as i64)))?;
    let mut tag = [0u8; IDENTIFIER_SIZE];
    file.read_exact(&mut tag)?;
    Ok(&tag == ID3V1_IDENTIFIER)
}

#[derive(Debug)]
pub struct TagReader {
    path: PathBuf,
    tag: Option<Tag>,
}

#[derive(Debug)]
pub struct TagWriter {
    path: PathBuf,
    tag: Option<Tag>,
}

/// ID3v1 tag implementation
#[derive(Debug, Default)]
pub struct Tag {
    pub title: [u8; TITLE_SIZE],
    pub artist: [u8; ARTIST_SIZE],
    pub album: [u8; ALBUM_SIZE],
    pub year: [u8; YEAR_SIZE],
    pub comment: [u8; COMMENT_SIZE],
    pub genre: [u8; GENRE_SIZE],
}

impl TagReader {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            tag: None,
        }
    }
}

impl TagWriter {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            tag: None,
        }
    }
}

impl TagReaderStrategy for TagReader {
    fn init(&mut self, path: &Path) -> Result<()> {
        self.path = path.to_path_buf();
        if has_id3v1_tag(path).unwrap_or(false) {
            self.tag = Some(Tag::read_from_file(path)?);
        }
        Ok(())
    }

    fn get_meta_entry(&self, _path: &Path, entry: &MetaEntry) -> Result<String> {
        if let Some(tag) = &self.tag {
            match entry {
                MetaEntry::Title => Ok(String::from_utf8_lossy(&tag.title).trim_end().to_string()),
                MetaEntry::Artist => Ok(String::from_utf8_lossy(&tag.artist).trim_end().to_string()),
                MetaEntry::Album => Ok(String::from_utf8_lossy(&tag.album).trim_end().to_string()),
                MetaEntry::Year => Ok(String::from_utf8_lossy(&tag.year).trim_end().to_string()),
                MetaEntry::Comment => Ok(String::from_utf8_lossy(&tag.comment).trim_end().to_string()),
                _ => Err(Error::EntryNotFound),
            }
        } else {
            Err(Error::TagNotFound)
        }
    }

    fn tag_type(&self) -> TagType {
        TagType::Id3v1
    }
}

impl TagWriterStrategy for TagWriter {
    fn init(&mut self, path: &Path) -> Result<()> {
        self.path = path.to_path_buf();
        if has_id3v1_tag(path).unwrap_or(false) {
            self.tag = Some(Tag::read_from_file(path)?);
        } else {
            self.tag = Some(Tag::new());
        }
        Ok(())
    }

    fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()> {
        let tag = self.tag.get_or_insert_with(Tag::new);
        match entry {
            MetaEntry::Title => tag.title[..value.len().min(TITLE_SIZE)].copy_from_slice(value.as_bytes()),
            MetaEntry::Artist => tag.artist[..value.len().min(ARTIST_SIZE)].copy_from_slice(value.as_bytes()),
            MetaEntry::Album => tag.album[..value.len().min(ALBUM_SIZE)].copy_from_slice(value.as_bytes()),
            MetaEntry::Year => tag.year[..value.len().min(YEAR_SIZE)].copy_from_slice(value.as_bytes()),
            MetaEntry::Comment => tag.comment[..value.len().min(COMMENT_SIZE)].copy_from_slice(value.as_bytes()),
            _ => return Ok(()),
        }
        Ok(())
    }

    fn save(&mut self) -> Result<()> {
        if let Some(tag) = &self.tag {
            tag.write_to_file(&self.path)?;
        }
        Ok(())
    }

    fn tag_type(&self) -> TagType {
        TagType::Id3v1
    }
}

impl Tag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let file_len = file.seek(SeekFrom::End(0))?;
        
        if file_len < ID3V1_TAG_SIZE as u64 {
            return Err(Error::TagNotFound);
        }

        file.seek(SeekFrom::End(-(ID3V1_TAG_SIZE as i64)))?;
        
        let mut tag_data = [0u8; ID3V1_TAG_SIZE];
        file.read_exact(&mut tag_data)?;
        
        if &tag_data[IDENTIFIER_OFFSET..IDENTIFIER_OFFSET + IDENTIFIER_SIZE] != ID3V1_IDENTIFIER {
            return Err(Error::TagNotFound);
        }

        let mut tag = Tag::new();
        tag.title.copy_from_slice(&tag_data[TITLE_OFFSET..TITLE_OFFSET + TITLE_SIZE]);
        tag.artist.copy_from_slice(&tag_data[ARTIST_OFFSET..ARTIST_OFFSET + ARTIST_SIZE]);
        tag.album.copy_from_slice(&tag_data[ALBUM_OFFSET..ALBUM_OFFSET + ALBUM_SIZE]);
        tag.year.copy_from_slice(&tag_data[YEAR_OFFSET..YEAR_OFFSET + YEAR_SIZE]);
        tag.comment.copy_from_slice(&tag_data[COMMENT_OFFSET..COMMENT_OFFSET + COMMENT_SIZE]);
        tag.genre.copy_from_slice(&tag_data[GENRE_OFFSET..GENRE_OFFSET + GENRE_SIZE]);

        Ok(tag)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)?;
        let file_len = file.seek(SeekFrom::End(0))?;
        
        if file_len < ID3V1_TAG_SIZE as u64 {
            return Err(Error::TagNotFound);
        }

        file.seek(SeekFrom::End(-(ID3V1_TAG_SIZE as i64)))?;
        
        let mut tag_data = [0u8; ID3V1_TAG_SIZE];
        tag_data[IDENTIFIER_OFFSET..IDENTIFIER_OFFSET + IDENTIFIER_SIZE].copy_from_slice(ID3V1_IDENTIFIER);
        
        tag_data[TITLE_OFFSET..TITLE_OFFSET + TITLE_SIZE].copy_from_slice(&self.title);
        tag_data[ARTIST_OFFSET..ARTIST_OFFSET + ARTIST_SIZE].copy_from_slice(&self.artist);
        tag_data[ALBUM_OFFSET..ALBUM_OFFSET + ALBUM_SIZE].copy_from_slice(&self.album);
        tag_data[YEAR_OFFSET..YEAR_OFFSET + YEAR_SIZE].copy_from_slice(&self.year);
        tag_data[COMMENT_OFFSET..COMMENT_OFFSET + COMMENT_SIZE].copy_from_slice(&self.comment);
        tag_data[GENRE_OFFSET..GENRE_OFFSET + GENRE_SIZE].copy_from_slice(&self.genre);

        file.write_all(&tag_data)?;
        Ok(())
    }
}
