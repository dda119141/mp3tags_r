use std::io;
use thiserror::Error;

/// Result type for the library
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the mp3tags_r library
use crate::validation::ValidationError;

#[derive(Error, Debug)]
pub enum Error {
    /// Invalid tag header
    #[error("Invalid tag header")]
    InvalidHeader,
    /// Invalid tag type
    #[error("Invalid tag type")]
    InvalidTagType,
    /// Error when opening or reading a file
    #[error("File error: {0}")]
    FileError(#[from] io::Error),

    /// Error when a tag is not found
    #[error("Tag not found")]
    TagNotFound,

    /// Error when a tag has an invalid version
    #[error("Invalid tag version: {0}")]
    InvalidTagVersion(String),

    /// Error when a tag has an invalid size
    #[error("Invalid tag size")]
    InvalidTagSize,

    /// Error when a frame ID is not found
    #[error("Frame ID not found: {0}")]
    FrameIdNotFound(String),

    /// Error when a frame ID is at an invalid position
    #[error("Frame ID at invalid position")]
    FrameIdInvalidPosition,

    /// Error when a frame has no payload length
    #[error("No frame payload length")]
    NoFramePayloadLength,

    /// Error when a frame length is larger than the tag length
    #[error("Frame length larger than tag length")]
    FrameLengthExceedsTagLength,

    /// Error when content length is larger than frame area
    #[error("Content length larger than frame area")]
    ContentLengthExceedsFrameArea,

    /// Error when payload start position is after payload end position
    #[error("Payload start position after payload end position")]
    PayloadPositionInvalid,

    /// Error when content is not printable
    #[error("Content is not printable")]
    NonPrintableContent,

    /// Error when renaming a file
    #[error("Error renaming file: {0}")]
    FileRenameError(String),
    
    /// Error when extending tag area
    #[error("Error extending tag area")]
    ExtendTagError,
    
    /// Error when writing ID3v1 tag
    #[error("Error writing ID3v1 tag: {0}")]
    Id3v1WriteError(String),
    
    /// Error when reading ID3v1 tag
    #[error("Error reading ID3v1 tag: {0}")]
    Id3v1ReadError(String),
    
    /// Error when validating ID3v1 tag field
    #[error("Invalid ID3v1 field: {0}")]
    Id3v1FieldError(String),
    
    /// Error when file is read-only
    #[error("File is read-only: {0}")]
    ReadOnlyFileError(String),
    
    /// Error when a meta entry is not supported by tag type
    #[error("Meta entry not supported by tag type: {0}")]
    UnsupportedMetaEntry(String),
    
    /// Generic error with message
    #[error("Other error: {0}")]
    Other(String),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),
    
    /// Error when a meta entry is not found
    #[error("Meta entry not found")]
    EntryNotFound,
}
