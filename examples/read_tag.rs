use std::env;
use std::path::Path;
use std::process;

use mp3tags_r::{get_title, get_artist, get_album, get_year, get_genre, get_comment, get_all_meta_entries};

#[derive(Default)]
struct TagOptions {
    album: bool,
    genre: bool,
    title: bool,
    artist: bool,
    year: bool,
    comment: bool,
    all_entries: bool,
}

impl TagOptions {
    fn is_empty(&self) -> bool {
        !(self.album || self.genre || self.title || self.artist || self.year || self.comment || self.all_entries)
    }
}

fn get_tag_value<P, F>(path: P, getter: F, field_name: &str) -> String
where
    P: AsRef<Path>,
    F: FnOnce(P) -> mp3tags_r::Result<String>,
{
    getter(path)
        .unwrap_or_else(|_| format!("N/A (no {} tag)", field_name))
}

fn read_tags_in_file<P: AsRef<Path>>(file_path: P, options: &TagOptions) {
    let path = file_path.as_ref();
    let filename = path.file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_else(|| "Unknown".into());
    
    if options.title {
        println!("Get title of file: {} : {}", filename, get_tag_value(path, get_title, "title"));
    }
    
    if options.artist {
        println!("Get artist of file: {} : {}", filename, get_tag_value(path, get_artist, "artist"));
    }
    
    if options.album {
        println!("Get album of file: {} : {}", filename, get_tag_value(path, get_album, "album"));
    }
    
    if options.year {
        println!("Get year of file: {} : {}", filename, get_tag_value(path, get_year, "year"));
    }
    
    if options.genre {
        println!("Get genre of file: {} : {}", filename, get_tag_value(path, get_genre, "genre"));
    }
    
    if options.comment {
        println!("Get comment of file: {} : {}", filename, get_tag_value(path, get_comment, "comment"));
    }
    
    if options.all_entries {
        println!("All meta entries for file: {}", filename);
        match get_all_meta_entries(path) {
            Ok(entries) => {
                for (entry, value) in entries {
                    println!("  {:?}: {}", entry, value);
                }
            }
            Err(e) => println!("  Error reading entries: {}", e),
        }
    }
}

fn read_tags<P: AsRef<Path>>(path: P, options: &TagOptions) {
    let path = path.as_ref();
    
    if !path.exists() {
        eprintln!("Path does not exist: {}", path.display());
        return;
    }
    
    if path.is_file() {
        read_tags_in_file(path, options);
    } else if path.is_dir() {
        // Read tags from all files in directory recursively
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                
                if file_path.is_file() {
                    if let Some(extension) = file_path.extension() {
                        if extension == "mp3" {
                            read_tags_in_file(&file_path, options);
                        }
                    }
                } else if file_path.is_dir() {
                    read_tags(&file_path, options);
                }
            }
        }
    }
}

fn print_usage() {
    println!("Usage: read_tag [OPTIONS] <FILE_OR_DIRECTORY>");
    println!();
    println!("This is a utility for reading tags from MP3 files.");
    println!();
    println!("OPTIONS:");
    println!("  -t, --title        Get the title frame content");
    println!("  -a, --artist       Get the artist frame content");
    println!("  -b, --album        Get the album frame content");
    println!("  -g, --genre        Get the genre frame content");
    println!("  -y, --year         Get the album release year");
    println!("  -c, --comment      Get the comment frame content");
    println!("  -e, --all-entries  Get all meta entries");
    println!("  -h, --help         Show this help message");
    println!();
    println!("ARGUMENTS:");
    println!("  <FILE_OR_DIRECTORY>  Specify file or directory to read tags from (REQUIRED)");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    let mut options = TagOptions::default();
    let mut file_path = String::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" | "--title" => options.title = true,
            "-a" | "--artist" => options.artist = true,
            "-b" | "--album" => options.album = true,
            "-g" | "--genre" => options.genre = true,
            "-y" | "--year" => options.year = true,
            "-c" | "--comment" => options.comment = true,
            "-e" | "--all-entries" => options.all_entries = true,
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            arg if !arg.starts_with('-') => {
                if file_path.is_empty() {
                    file_path = arg.to_string();
                } else {
                    eprintln!("Error: Multiple file paths specified");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unknown option: {}", args[i]);
                print_usage();
                process::exit(1);
            }
        }
        i += 1;
    }
    
    if file_path.is_empty() {
        eprintln!("Error: No file or directory specified");
        print_usage();
        process::exit(1);
    }
    
    if options.is_empty() {
        eprintln!("Error: No tag options specified");
        print_usage();
        process::exit(1);
    }
    
    read_tags(&file_path, &options);
}
