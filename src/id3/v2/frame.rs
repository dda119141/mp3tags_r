use crate::error::{Error, Result};

/// ID3v2 frame flags
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
pub struct FrameFlags {
    pub tag_alter_preservation: bool,
    pub file_alter_preservation: bool,
    pub read_only: bool,
    pub compression: bool,
    pub encryption: bool,
    pub grouping_identity: bool,
}

/// ID3v2 frame implementation
#[derive(Debug, Clone)]
pub struct Frame {
    pub id: String,
    pub content: String,
    data: Vec<u8>,
}

impl Frame {
    pub fn parse(data: &[u8], _version: u8) -> Result<Self> {
        if data.len() < 10 {
            return Err(Error::InvalidHeader);
        }
        
        let mut header = [0u8; 10];
        header.copy_from_slice(&data[..10]);
        
        // Parse frame header manually since FrameHeader doesn't exist yet
        let id = String::from_utf8_lossy(&header[0..4]).to_string();
        let size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        let frame_data = data[10..10 + size as usize].to_vec();
        
        // ID3v2 text frames start with a text encoding byte
        let content = if frame_data.is_empty() {
            String::new()
        } else {
            // Skip the first byte (text encoding) and parse the rest as text
            String::from_utf8_lossy(&frame_data[1..]).to_string()
        };
        
        Ok(Self {
            id,
            content,
            data: frame_data,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(10 + self.data.len());
        let mut header = [0u8; 10];
        header[0..4].copy_from_slice(self.id.as_bytes());
        let size_bytes = (self.data.len() as u32).to_be_bytes();
        header[4..8].copy_from_slice(&size_bytes);
        // flags are already 0
        bytes.extend_from_slice(&header);
        bytes.extend_from_slice(&self.data);
        bytes
    }

    pub fn new(id: &str, content: &str) -> Self {
        // ID3v2 text frames start with a text encoding byte (0x00 = ISO-8859-1)
        let mut data = vec![0x00];
        data.extend_from_slice(content.as_bytes());
        Self {
            id: id.to_string(),
            content: content.to_string(),
            data,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn total_size(&self) -> usize {
        10 + self.data.len() // Header size (10) + data size
    }

    pub fn size(&self) -> usize {
        10 + self.data.len() // Header (10 bytes) + data
    }
}

