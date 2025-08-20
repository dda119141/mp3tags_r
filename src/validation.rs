use thiserror::Error;
use crate::meta_entry::MetaEntry;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Value exceeds max length: {0}")]
    MaxLengthExceeded(String),
    #[error("Invalid characters in {0}")]
    InvalidCharacters(String),
    #[error("Invalid year format")]
    InvalidYear,
}

pub trait BaseValidator {
    fn validate_length(&self, entry: &MetaEntry, value: &str) -> Result<(), ValidationError> {
        let max_len = match entry {
            MetaEntry::Title | MetaEntry::Artist | MetaEntry::Album => 256,
            MetaEntry::Comment => 512,
            MetaEntry::Year => 4,
            _ => return Ok(()),
        };
        
        if value.len() > max_len {
            return Err(ValidationError::MaxLengthExceeded(entry.to_string()));
        }
        Ok(())
    }

    fn validate_chars(&self, entry: &MetaEntry, value: &str) -> Result<(), ValidationError> {
        match entry {
            MetaEntry::Year if !value.chars().all(|c| c.is_ascii_digit()) => {
                Err(ValidationError::InvalidCharacters(entry.to_string()))
            }
            _ => Ok(())
        }
    }
}

pub trait Id3v2Validator: BaseValidator {
    fn validate_frame(&self, frame_id: &str, value: &str) -> Result<(), ValidationError> {
        // Convert frame ID to MetaEntry based on ID3v2 frame ID mapping
        let entry = match frame_id {
            "TIT2" => MetaEntry::Title,
            "TPE1" => MetaEntry::Artist,
            "TALB" => MetaEntry::Album,
            "TYER" => MetaEntry::Year,
            "COMM" => MetaEntry::Comment,
            "TCOM" => MetaEntry::Composer,
            _ => return Ok(()), // Unknown frame IDs are allowed
        };
        self.validate_length(&entry, value)?;
        self.validate_chars(&entry, value)
    }
}

pub trait ApeValidator: BaseValidator {
    fn validate_item(&self, key: &str, value: &str) -> Result<(), ValidationError> {
        // Convert APE key to MetaEntry based on common APE key mapping
        let entry = match key.to_uppercase().as_str() {
            "TITLE" => MetaEntry::Title,
            "ARTIST" => MetaEntry::Artist,
            "ALBUM" => MetaEntry::Album,
            "YEAR" => MetaEntry::Year,
            "COMMENT" => MetaEntry::Comment,
            "COMPOSER" => MetaEntry::Composer,
            _ => return Ok(()), // Unknown keys are allowed
        };
        self.validate_length(&entry, value)?;
        self.validate_chars(&entry, value)
    }
}

pub struct StandardValidator;

impl BaseValidator for StandardValidator {}
impl Id3v2Validator for StandardValidator {}
impl ApeValidator for StandardValidator {}
