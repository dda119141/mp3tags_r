pub mod constants;
pub mod v1;
pub mod v2;

pub use v1::tag::{TagReader as Id3v1TagReader, TagWriter as Id3v1TagWriter};
pub use v2::tag::{TagReader as Id3v2TagReader, TagWriter as Id3v2TagWriter};
pub use v2::version::Version as Id3v2Version;
