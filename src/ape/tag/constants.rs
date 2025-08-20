/// APE tag constants
pub const APE_TAG_IDENTIFIER: &[u8] = b"APETAGEX";
pub const APE_TAG_FOOTER_SIZE: usize = 32;
pub const APE_TAG_HEADER_SIZE: usize = 32;
pub const APE_VERSION: u32 = 2000;

// APE tag flags
pub const APE_TAG_HAS_HEADER: u32 = 1 << 31;
pub const APE_TAG_HAS_FOOTER: u32 = 1 << 30;
pub const APE_TAG_IS_HEADER: u32 = 1 << 29;
