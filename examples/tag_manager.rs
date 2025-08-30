// No HashMap needed anymore
use std::{collections::HashMap, env};
use std::path::Path;
use std::process;

use mp3tags_r::{TagReader, TagWriter, MetaEntry, TagType, Result, Error};

fn print_usage() {
    println!("MP3 Tag Manager - A simple tool to read and modify MP3 tags");
    println!();
    println!("Usage:");
    println!("  tag_manager <command> <file> [options]");
    println!();
    println!("Commands:");
    println!("  read        Read all tags from the MP3 file");
    println!("  get         Get a specific tag from the MP3 file");
    println!("  set         Set a tag in the MP3 file");
    println!("  remove      Remove a tag from the MP3 file");
    println!("  clear       Remove all tags from the MP3 file");
    println!();
    println!("Options:");
    println!("  For 'get' command:");
    println!("    <tag>     The tag to get (title, artist, album, year, genre, track, comment)");
    println!();
    println!("  For 'set' command:");
    println!("    <tag>     The tag to set (title, artist, album, year, genre, track, comment)");
    println!("    <value>   The value to set for the tag");
    println!("    --type    The tag type to use (ape, id3v2, id3v1, default: use existing or ape)");
    println!();
    println!("  For 'remove' command:");
    println!("    <tag>     The tag to remove (title, artist, album, year, genre, track, comment)");
    println!();
    println!("Examples:");
    println!("  tag_manager read song.mp3");
    println!("  tag_manager get song.mp3 title");
    println!("  tag_manager set song.mp3 title \"My Song Title\"");
    println!("  tag_manager set song.mp3 artist \"Artist Name\" --type id3v2");
    println!("  tag_manager remove song.mp3 comment");
    println!("  tag_manager clear song.mp3");
}

fn parse_meta_entry(tag: &str) -> std::result::Result<MetaEntry, String> {
    match tag.to_lowercase().as_str() {
        "title" => Ok(MetaEntry::Title),
        "artist" => Ok(MetaEntry::Artist),
        "album" => Ok(MetaEntry::Album),
        "year" => Ok(MetaEntry::Year),
        "genre" => Ok(MetaEntry::Genre),
        "track" => Ok(MetaEntry::Track),
        "comment" => Ok(MetaEntry::Comment),
        "composer" => Ok(MetaEntry::Composer),
        _ => Err(format!("Unknown tag: {}", tag)),
    }
}

fn read_tags(file_path: &Path) -> Result<()> {
    // Create a new tag reader
    let reader = TagReader::new(file_path)?;
    
    // Get all meta entries
    let entries: HashMap<MetaEntry, String> = reader.get_all_meta_entries();
    
    // error handling check if the map is empty 
    if entries.is_empty() {
        return Err(Error::Other("No tags found in the file".to_string()));
    }

    // Print tags in a specific order
    let ordered_entries = [
            MetaEntry::Title,
            MetaEntry::Artist,
            MetaEntry::Album,
            MetaEntry::Year,
            MetaEntry::Genre,
            MetaEntry::Track,
            MetaEntry::Comment,
            MetaEntry::Composer,
        ];
        
    for entry in ordered_entries.iter() {
            if let Some(value) = entries.get(entry) {
                println!("{:<10}: {}", format!("{:?}", entry), value);
            }
        }
        
    // Print any other tags not in the ordered list
    for (entry, value) in entries.iter() {
        if !ordered_entries.contains(entry) {
            println!("{:<10}: {}", format!("{:?}", entry), value);
        }
    }
    
    Ok(())
}

fn get_tag(file_path: &Path, tag: &str) -> Result<()> {
    // Parse the meta entry
    let meta_entry = parse_meta_entry(tag).map_err(|e| Error::Other(format!("Invalid tag: {}", e)))?;
    
    // Create a new tag reader
    let reader = TagReader::new(file_path)?;
    
    // Get the tag value
    let value = reader.get_meta_entry(&meta_entry)?;
    
    println!("{}: {}", tag, value);
    Ok(())
}

fn set_tag(file_path: &Path, tag: &str, value: &str, tag_type_str: Option<&str>) -> Result<()> {
    // Parse the meta entry
    let meta_entry = parse_meta_entry(tag).map_err(|e| Error::Other(format!("Invalid tag: {}", e)))?;
    
    // Parse tag type from argument or use default
    let tag_type = if let Some(type_str) = tag_type_str {
        match type_str.to_lowercase().as_str() {
            "ape" => TagType::Ape,
            "id3v1" => TagType::Id3v1,
            "id3v2" => TagType::Id3v2,
            _ => return Err(Error::Other(format!("Invalid tag type: {}", type_str)))
        }
    } else {
        TagType::Id3v2 // Default to ID3v2
    };
    
    let mut writer = TagWriter::new(file_path, tag_type)?;
    
    // Set the meta entry
    writer.set_meta_entry(&meta_entry, value)?;
    
    println!("Tag '{}' set to '{}' using {:?} format.", tag, value, tag_type);
    Ok(())
}

fn remove_tag(file_path: &Path, tag: &str) -> Result<()> {
    // Parse the meta entry
    let meta_entry = parse_meta_entry(tag).map_err(|e| Error::Other(format!("Invalid tag: {}", e)))?;
    
    // Create a new tag writer
    let mut writer = TagWriter::new(file_path, TagType::Id3v2)?;
    
    // For now, we'll just set the entry to an empty string
    // This is a simple way to "remove" the tag
    writer.set_meta_entry(&meta_entry, "")?;
    
    println!("Tag '{}' removed.", tag);
    Ok(())
}

fn clear_tags(file_path: &Path) -> Result<()> {
    // Create a new tag writer
    let mut writer = TagWriter::new(file_path, TagType::Id3v2)?;
    
    // For each meta entry type, set it to empty string
    let entries = [
        MetaEntry::Title,
        MetaEntry::Artist,
        MetaEntry::Album,
        MetaEntry::Year,
        MetaEntry::Genre,
        MetaEntry::Track,
        MetaEntry::Comment,
        MetaEntry::Composer,
    ];
    
    // Track any errors that occur during tag clearing
    let mut errors = Vec::new();
    
    // Try to clear each tag
    for entry in &entries {
        if let Err(e) = writer.set_meta_entry(entry, "") {
            errors.push(format!("Failed to clear {:?}: {}", entry, e));
        }
    }
    
    // If any errors occurred, return them as a combined error
    if !errors.is_empty() {
        return Err(Error::Other(format!("Some tags could not be removed: {}", errors.join(", "))));
    }
    
    println!("All tags removed.");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }
    
    let command = &args[1];
    let file_path = Path::new(&args[2]);
    
    if !file_path.exists() {
        eprintln!("File not found: {}", file_path.display());
        process::exit(1);
    }
    
    match command.as_str() {
        "read" => {
            if let Err(e) = read_tags(file_path) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        "get" => {
            if args.len() < 4 {
                eprintln!("Missing tag name for 'get' command.");
                print_usage();
                process::exit(1);
            }
            
            let tag = &args[3];
            if let Err(e) = get_tag(file_path, tag) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        "set" => {
            if args.len() < 5 {
                eprintln!("Missing tag name or value for 'set' command.");
                print_usage();
                process::exit(1);
            }
            
            let tag = &args[3];
            let value = &args[4];
            
            // Check for tag type option
            let mut tag_type = None;
            if args.len() > 5 && args[5] == "--type" && args.len() > 6 {
                tag_type = Some(args[6].as_str());
            }
            
            if let Err(e) = set_tag(file_path, tag, value, tag_type) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        "remove" => {
            if args.len() < 4 {
                eprintln!("Missing tag name for 'remove' command.");
                print_usage();
                process::exit(1);
            }
            
            let tag = &args[3];
            if let Err(e) = remove_tag(file_path, tag) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        "clear" => {
            if let Err(e) = clear_tags(file_path) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}
