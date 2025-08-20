/// ID3v1 tag constants
pub const TAG_IDENTIFIER: &[u8] = b"TAG";
pub const TAG_LENGTH: usize = 128;

// Field lengths
pub const TITLE_LENGTH: usize = 30;
pub const ARTIST_LENGTH: usize = 30;
pub const ALBUM_LENGTH: usize = 30;
pub const YEAR_LENGTH: usize = 4;
pub const COMMENT_LENGTH: usize = 30;

// Field offsets
pub const TITLE_OFFSET: usize = 3;
pub const ARTIST_OFFSET: usize = TITLE_OFFSET + TITLE_LENGTH;
pub const ALBUM_OFFSET: usize = ARTIST_OFFSET + ARTIST_LENGTH;
pub const YEAR_OFFSET: usize = ALBUM_OFFSET + ALBUM_LENGTH;
pub const COMMENT_OFFSET: usize = YEAR_OFFSET + YEAR_LENGTH;
pub const GENRE_OFFSET: usize = COMMENT_OFFSET + COMMENT_LENGTH;
