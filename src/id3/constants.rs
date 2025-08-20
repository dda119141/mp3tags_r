/// ID3v2 header size
pub const HEADER_SIZE: usize = 10;

/// ID3v2 identifier
pub const ID3V2_IDENTIFIER: &[u8] = b"ID3";

/// ID3v1 tag size
pub const ID3V1_TAG_SIZE: usize = 128;

/// ID3v1 identifier
pub const ID3V1_IDENTIFIER: &[u8] = b"TAG";

/// ID3v2 flag for extended header
pub const ID3V2_FLAG_EXTENDED_HEADER: u8 = 0x40;
