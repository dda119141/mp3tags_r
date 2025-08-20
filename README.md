# mp3tags_r

A comprehensive Rust library for reading and writing MP3 metadata tags, supporting ID3v1, ID3v2, and APE tag formats.

## Features

- **Multiple Tag Format Support**
  - ID3v1 tags (read/write)
  - ID3v2.3 tags (read/write)
  - APE tags (read/write)
- **Automatic Tag Detection** - Intelligently detects and prioritizes tag formats
- **Clean API Design** - Uses strategy and template patterns for extensibility
- **Memory Efficient** - On-demand frame lookup for ID3v2 tags
- **Error Handling** - Comprehensive error types with detailed messages
- **Cross-Platform** - Works on Windows, macOS, and Linux
- **Zero Unsafe Code** - Built entirely with safe Rust

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mp3tags_r = "0.1.0"
```

## Quick Start

### Reading Tags

```rust
use mp3tags_r::{TagReader, MetaEntry, Result};

fn main() -> Result<()> {
    let path = "path/to/your/file.mp3";
    
    // Create a tag reader
    let reader = TagReader::new(path)?;
    
    // Read individual metadata entries
    let title = reader.get_meta_entry(&MetaEntry::Title)?;
    println!("Title: {}", title);
    
    let artist = reader.get_meta_entry(&MetaEntry::Artist)?;
    println!("Artist: {}", artist);
    
    let album = reader.get_meta_entry(&MetaEntry::Album)?;
    println!("Album: {}", album);
    
    Ok(())
}
```

### Writing Tags

```rust
use mp3tags_r::{TagWriter, MetaEntry, Result};

fn main() -> Result<()> {
    let path = "path/to/your/file.mp3";
    
    // Create a tag writer
    let mut writer = TagWriter::new(path)?;
    
    // Set metadata entries (automatically saved)
    writer.set_meta_entry(&MetaEntry::Title, "My Song Title")?;
    writer.set_meta_entry(&MetaEntry::Artist, "My Artist")?;
    writer.set_meta_entry(&MetaEntry::Album, "My Album")?;
    writer.set_meta_entry(&MetaEntry::Year, "2024")?;
    writer.set_meta_entry(&MetaEntry::Genre, "Rock")?;
    
    println!("Tags updated successfully!");
    Ok(())
}
```

## Supported Metadata Fields

The library supports the following metadata entries through the `MetaEntry` enum:

- `Title` - Song title
- `Artist` - Primary artist/performer
- `Album` - Album name
- `Year` - Release year
- `Genre` - Music genre
- `Track` - Track number
- `Comment` - Comments
- `Composer` - Song composer
- `Custom(String)` - Custom fields (e.g., "AlbumArtist", "Disc")

## Advanced Usage

### Tag Priority and Detection

The library automatically detects available tag formats and uses a priority system:
1. **APE tags** (highest priority)
2. **ID3v2 tags** (medium priority)  
3. **ID3v1 tags** (lowest priority)

```rust
use mp3tags_r::{TagReader, TagType};

fn main() -> Result<()> {
    let mut reader = TagReader::new("file.mp3")?;
    
    // Check which tag types are present
    if reader.is_present(TagType::Ape) {
        println!("APE tag found");
    }
    if reader.is_present(TagType::Id3v2) {
        println!("ID3v2 tag found");
    }
    if reader.is_present(TagType::Id3v1) {
        println!("ID3v1 tag found");
    }
    
    Ok(())
}
```

### Error Handling

```rust
use mp3tags_r::{TagReader, MetaEntry, Error};

fn read_tags_safely(path: &str) {
    match TagReader::new(path) {
        Ok(mut reader) => {
            match reader.get_meta_entry(&MetaEntry::Title) {
                Ok(Some(title)) => println!("Title: {}", title),
                Ok(None) => println!("No title found"),
                Err(Error::FileError(e)) => eprintln!("File error: {}", e),
                Err(Error::TagNotFound) => eprintln!("No tags found"),
                Err(e) => eprintln!("Other error: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to create reader: {}", e),
    }
}
```

### Custom Fields

```rust
use mp3tags_r::{TagWriter, MetaEntry};

fn main() -> Result<()> {
    let mut writer = TagWriter::new("file.mp3")?;
    
    // Set standard fields
    writer.set_meta_entry(&MetaEntry::Title, "Song Title")?;
    
    // Set custom fields
    writer.set_meta_entry(&MetaEntry::Custom("AlbumArtist".to_string()), "Various Artists")?;
    writer.set_meta_entry(&MetaEntry::Custom("Disc".to_string()), "1/2")?;
    writer.set_meta_entry(&MetaEntry::Custom("BPM".to_string()), "120")?;
    
    writer.save()?;
    Ok(())
}
```

## Command-Line Tools

The library includes several command-line utilities in the `examples/` directory:

### Reading Tags (`read_tag`)

```bash
# Read all tags from a file
cargo run --example read_tag -- /path/to/file.mp3

# Read specific tag type
cargo run --example read_tag -- --id3v2 /path/to/file.mp3
```

### Writing Tags (`write_tag`)

```bash
# Set multiple tags
cargo run --example write_tag -- \
    --title "My Song" \
    --artist "My Artist" \
    --album "My Album" \
    --year "2024" \
    /path/to/file.mp3

# Process entire directory
cargo run --example write_tag -- \
    --artist "Various Artists" \
    /path/to/music/directory/
```

### Directory Management (`handle_directory`)

```bash
# Find directories with only small files
cargo run --example handle_directory -- --directory /path/to/music --empty

# Remove directories with only small files
cargo run --example handle_directory -- --directory /path/to/music --remove
```

## API Reference

### TagReader

```rust
impl TagReader {
    // Create a new tag reader for the specified file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self>;
    
    // Get a metadata entry value
    pub fn get_meta_entry(&mut self, entry: &MetaEntry) -> Result<Option<String>>;
    
    // Check if a specific tag type is present
    pub fn is_present(&self, tag_type: TagType) -> bool;
}
```

### TagWriter

```rust
impl TagWriter {
    // Create a new tag writer for the specified file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self>;
    
    // Set a metadata entry value
    pub fn set_meta_entry(&mut self, entry: &MetaEntry, value: &str) -> Result<()>;
    
    // Save all changes to the file
    pub fn save(&mut self) -> Result<()>;
}
```

### Error Types

```rust
pub enum Error {
    FileError(std::io::Error),    // File I/O errors
    TagNotFound,                  // No tag found
    EntryNotFound,               // Specific entry not found
    Other(String),               // Other errors with description
}
```

## Performance Considerations

- **Lazy Loading**: Tags are only read when accessed
- **Minimal Memory Usage**: Large files are processed in chunks
- **Efficient Updates**: Only modified tags are rewritten
- **Atomic Operations**: File updates are atomic (temp file + rename)

## Supported File Types

While named `mp3tags_r`, the library works with any file format that supports the tag types:

- **MP3 files** (.mp3) - ID3v1, ID3v2, and APE tags
- **APE files** (.ape) - APE tags


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Examples

Check the `examples/` directory for complete working examples:

- **`read_tag.rs`** - Comprehensive tag reading with different formats
- **`write_tag.rs`** - Batch tag writing with command-line interface  
- **`handle_directory.rs`** - Directory processing and cleanup utilities
- **`tag_manager.rs`** - Advanced tag management operations

## License

This project is licensed under the MIT License - see the LICENSE file for details.
