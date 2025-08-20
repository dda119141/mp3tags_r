use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;

use crate::error::{Error, Result};
use crate::id3::constants::*;
use crate::id3::v2::frame::Frame;
use crate::id3::v2::frame_mapping::{v2_0, v3_v4};
use crate::id3::v2::header::Header;
use crate::id3::v2::util::has_id3v2_tag;
use crate::id3::v2::version::Version;
use crate::meta_entry::MetaEntry;
use crate::tag::{TagReaderStrategy, TagType, TagWriterStrategy};

/// Read a specific frame from ID3v2 tag 
fn get_specific_frame(path: &Path, target_frame_id: &str) -> Result<Frame> {
    let mut file = File::open(path)?;
    let mut header_buf = [0u8; HEADER_SIZE];
    file.read_exact(&mut header_buf)?;

    let header = Header::parse(&header_buf)?;
    if !header.is_valid() {
        return Err(Error::InvalidHeader);
    }

    let tag_size = header.size;
    let mut tag_buf = vec![0u8; tag_size as usize];
    file.read_exact(&mut tag_buf)?;

    let mut offset = 0;
    while offset < tag_size as usize {
        let frame = Frame::parse(&tag_buf[offset..], header.version)?;
        if frame.is_empty() {
            break;
        }

        if frame.id == target_frame_id {
            return Ok(frame);
        }

        offset += frame.total_size();
    }

    return Err(Error::FrameIdNotFound(target_frame_id.to_string()));
}

/// Get the version of an ID3v2 tag
fn get_id3v2_version(path: &Path) -> Result<Version> {
    let mut file = File::open(path)?;
    let mut header_buf = [0u8; HEADER_SIZE];
    file.read_exact(&mut header_buf)?;

    let header = Header::parse(&header_buf)?;
    if !header.is_valid() {
        return Err(Error::InvalidHeader);
    }

    Ok(header.version.into())
}

#[derive(Debug)]
pub struct TagReader {
    tag: Option<Tag>,
}

impl TagReader {
    pub fn new() -> Self {
        Self { tag: None }
    }
}

impl TagReaderStrategy for TagReader {
    fn init(&mut self, path: &Path) -> Result<()> {
        self.tag = if has_id3v2_tag(path).unwrap_or(false) {
            let version = get_id3v2_version(path)?;
            Some(Tag {
                version,
                flags: 0,
                frames: HashMap::new(),
            })
        } else {
            None
        };
        Ok(())
    }

    fn get_meta_entry(&self, path: &Path, entry: &MetaEntry) -> Result<Option<String>> {
        // Check if we have an ID3v2 tag at all
        if !has_id3v2_tag(path).unwrap_or(false) {
            return Ok(None);
        }

        // Get the version to determine the correct frame ID
        let version = get_id3v2_version(path)?;
        let frame_id = get_frame_id_for_version(entry, version);
        
        if let Some(id) = frame_id {
            match get_specific_frame(path, id) {
                Ok(frame) => Ok(Some(frame.content)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn tag_type(&self) -> TagType {
        TagType::Id3v2
    }
}

#[derive(Debug)]
pub struct TagWriter {
    path: PathBuf,
}

impl TagWriter {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }

    fn write_tag(&self, tag: &Tag) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)?;
        
        let header = Header::new(tag.version.into());
        
        let mut frame_data = Vec::new();
        for frames in tag.frames.values() {
            for frame in frames {
            frame_data.extend_from_slice(&frame.to_bytes());
            }
        }
        
        let mut header = header;
        header.size = frame_data.len() as u32;
        header.flags = tag.flags;
        
        file.seek(SeekFrom::Start(0))?;
        file.write_all(&header.to_bytes())?;
        file.write_all(&frame_data)?;
        
        Ok(())
    }

    fn read_existing_tag(&self) -> Result<Tag> {
        let mut file = File::open(&self.path)?;
        let mut header_buf = [0u8; HEADER_SIZE];
        file.read_exact(&mut header_buf)?;

        let header = Header::parse(&header_buf)?;
        if !header.is_valid() {
            return Err(Error::InvalidHeader);
        }

        let tag_size = header.size;
        let mut tag_buf = vec![0u8; tag_size as usize];
        file.read_exact(&mut tag_buf)?;

        let mut frames = HashMap::new();
        let mut offset = 0;
        while offset < tag_size as usize {
            let frame = Frame::parse(&tag_buf[offset..], header.version)?;
            if frame.is_empty() {
                break;
            }

            let frame_size = frame.total_size();
            frames.insert(frame.id.to_string(), vec![frame]);

            offset += frame_size;
        }

        Ok(Tag {
            version: header.version.into(),
            flags: header.flags,
            frames,
        })
    }
}

impl TagWriterStrategy for TagWriter {
    fn init(&mut self, path: &Path) -> Result<()> {
        self.path = path.to_path_buf();
        Ok(())
    }

    fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()> {
        let version = if has_id3v2_tag(&self.path).unwrap_or(false) {
            get_id3v2_version(&self.path)?
        } else {
            Version::V3
        };

        let frame_id = get_frame_id_for_version(entry, version)
            .ok_or_else(|| Error::Other(format!("No frame mapping for entry: {}", entry)))?;

        let frame = Frame::new(frame_id, value);
        
        // Read existing tag or create new one
        let mut tag = if has_id3v2_tag(&self.path).unwrap_or(false) {
            // Read existing tag to preserve other frames
            self.read_existing_tag()?
        } else {
            // Create new tag if none exists
            Tag {
                version,
                flags: 0,
                frames: HashMap::new(),
            }
        };

        // Update or insert the specific frame
        tag.frames.insert(frame_id.to_string(), vec![frame]);

        self.write_tag(&tag)
    }

    fn save(&mut self) -> Result<()> {
        Ok(())
    }

    fn tag_type(&self) -> TagType {
        TagType::Id3v2
    }
}

/// ID3v2 tag implementation
#[derive(Debug)]
pub struct Tag {
    version: Version,
    flags: u8,
    frames: HashMap<String, Vec<Frame>>,
}

// Rest of ID3v2 implementation moved here...

fn get_frame_id_for_version(entry: &MetaEntry, version: Version) -> Option<&'static str> {
    match version {
        Version::V2 => v2_0::get_frame_id(entry),
        Version::V3 | Version::V4 => v3_v4::get_frame_id(entry),
    }
}
