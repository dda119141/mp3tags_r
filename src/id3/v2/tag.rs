use std::collections::HashMap;
use std::fmt::Debug;
use log::{warn};
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

const FRAME_HEADER_SIZE: usize = 10;
const FRAME_ID_SIZE: usize = 4;

/// Read all frames from an ID3v2 tag
fn read_tag(path: &Path) -> Result<Tag> {
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

    let mut frames = HashMap::new();
    let mut offset = 0;
    while offset < tag_size as usize {
        // Check if we have enough bytes for a frame header
        if offset + FRAME_HEADER_SIZE > tag_buf.len() {
            break;
        }

        // Security: Check that the frame header is not pointing outside the tag
        let size_bytes = [tag_buf[offset + 4], tag_buf[offset + 5], tag_buf[offset + 6], tag_buf[offset + 7]];
        let frame_size = u32::from_be_bytes(size_bytes) as usize;
        if offset + FRAME_HEADER_SIZE + frame_size > tag_buf.len() {
            // The frame size is invalid, stop parsing
            warn!("Invalid frame size at offset {}", offset);
            break;
        }

        // Check for empty frame (all zeros)
        // If the frame ID is all zeros, it's an empty frame
        // This can happen if the tag is truncated or corrupted
        // Stop parsing if we reach an empty frame
        if tag_buf[offset..offset + FRAME_ID_SIZE].iter().all(|&b| b == 0) {
            warn!("Empty zeroed frame found at offset {}", offset);
            break;
        }

        let frame = Frame::parse(&tag_buf[offset..], header.version)?;
        if frame.is_empty() {
            warn!("Empty frame found at offset {}", offset);
            break;
        }

        let frame_size = frame.total_size();
        if frame_size == 0 {
            warn!("Invalid frame size at offset {}", offset);
            break;
        }

        frames.entry(frame.id.clone()).or_insert_with(Vec::new).push(frame);
        offset += frame_size;
    }

    Ok(Tag {
        version: header.version.into(),
        flags: header.flags,
        frames,
    })
}

#[derive(Debug)]
pub struct TagReader {
    tag: Option<Tag>,
}

impl Default for TagReader {
    fn default() -> Self {
        Self::new()
    }
}

impl TagReader {
    pub fn new() -> Self {
        Self { tag: None }
    }
}

impl TagReaderStrategy for TagReader {
    fn init(&mut self, path: &Path) -> Result<()> {
        self.tag = if has_id3v2_tag(path).unwrap_or(false) {
            Some(read_tag(path)?)
        } else {
            None
        };
        Ok(())
    }

    fn get_meta_entry(&self, _path: &Path, entry: &MetaEntry) -> Result<String> {
        // Use the cached tag info from init()
        let tag = self.tag.as_ref().ok_or(Error::TagNotFound)?;
        
        // Use the cached version instead of re-reading the file
        let frame_id = get_frame_id_for_version(entry, tag.version);
        
        if let Some(id) = frame_id {
            if let Some(frames) = tag.frames.get(id) {
                if let Some(frame) = frames.first() {
                    return Ok(frame.content.clone());
                }
            }
        }
        Err(Error::EntryNotFound)
    }

    fn tag_type(&self) -> TagType {
        TagType::Id3v2
    }
}

#[derive(Debug)]
pub struct TagWriter {
    path: PathBuf,
}

impl Default for TagWriter {
    fn default() -> Self {
        Self::new()
    }
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
            // Security: Check that the frame header is not pointing outside the tag
            if offset + FRAME_HEADER_SIZE > tag_buf.len() {
                break;
            }
            let size_bytes = [tag_buf[offset + 4], tag_buf[offset + 5], tag_buf[offset + 6], tag_buf[offset + 7]];
            let frame_size_from_header = u32::from_be_bytes(size_bytes) as usize;
            if offset + FRAME_HEADER_SIZE + frame_size_from_header > tag_buf.len() {
                // The frame size is invalid, stop parsing
                break;
            }

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
            // If a tag exists, read its version to ensure we don't downgrade it.
            let existing_tag = self.read_existing_tag()?;
            existing_tag.version
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

fn get_frame_id_for_version(entry: &MetaEntry, version: Version) -> Option<&'static str> {
    match version {
        Version::V2 => v2_0::get_frame_id(entry),
        Version::V3 | Version::V4 => v3_v4::get_frame_id(entry),
    }
}
