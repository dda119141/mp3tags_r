use std::env;
use std::path::Path;
use std::process;

use mp3tags_r::{TagWriter, MetaEntry, Result, Error};

#[derive(Default)]
struct TagOptions {
    album: Option<String>,
    genre: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    year: Option<String>,
    comment: Option<String>,
}

impl TagOptions {
    fn is_empty(&self) -> bool {
        self.album.is_none() && self.genre.is_none() && self.title.is_none() 
            && self.artist.is_none() && self.year.is_none() && self.comment.is_none()
    }
}

fn set_tag_value<P>(writer: &mut TagWriter, path: P, entry: &MetaEntry, value: &str, field_name: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let filename = path.file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_else(|| "Unknown".into());
    
    println!("Set {} of file: {} : {}", field_name, filename, value);
    writer.set_meta_entry(entry, value)?;
    Ok(())
}

fn change_tags_in_file<P: AsRef<Path>>(file_path: P, options: &TagOptions) -> Result<()> {
    let path = file_path.as_ref();
    
    if !path.exists() {
        return Err(Error::Other(format!("File does not exist: {}", path.display())));
    }
    
    let mut writer = TagWriter::new(path)?;
    let mut changes_made = false;
    
    if let Some(ref title) = options.title {
        set_tag_value(&mut writer, path, &MetaEntry::Title, title, "title")?;
        changes_made = true;
    }
    
    if let Some(ref artist) = options.artist {
        set_tag_value(&mut writer, path, &MetaEntry::Artist, artist, "artist")?;
        changes_made = true;
    }
    
    if let Some(ref album) = options.album {
        set_tag_value(&mut writer, path, &MetaEntry::Album, album, "album")?;
        changes_made = true;
    }
    
    if let Some(ref genre) = options.genre {
        set_tag_value(&mut writer, path, &MetaEntry::Genre, genre, "genre")?;
        changes_made = true;
    }
    
    if let Some(ref year) = options.year {
        set_tag_value(&mut writer, path, &MetaEntry::Year, year, "year")?;
        changes_made = true;
    }
    
    if let Some(ref comment) = options.comment {
        set_tag_value(&mut writer, path, &MetaEntry::Comment, comment, "comment")?;
        changes_made = true;
    }
    
    if changes_made {
        let filename = path.file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| "Unknown".into());
        println!("All changes applied successfully to: {}", filename);
    }
    
    Ok(())
}

fn write_tags<P: AsRef<Path>>(path: P, options: &TagOptions) {
    let path = path.as_ref();
    
    if !path.exists() {
        eprintln!("Path does not exist: {}", path.display());
        return;
    }
    
    if path.is_file() {
        if let Err(e) = change_tags_in_file(path, options) {
            eprintln!("Error changing tags in file {}: {}", path.display(), e);
        }
    } else if path.is_dir() {
        // Change tags in all MP3 files in directory recursively
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                
                if file_path.is_file() {
                    if let Some(extension) = file_path.extension() {
                        if extension == "mp3" {
                            if let Err(e) = change_tags_in_file(&file_path, options) {
                                eprintln!("Error changing tags in file {}: {}", file_path.display(), e);
                            }
                        }
                    }
                } else if file_path.is_dir() {
                    write_tags(&file_path, options);
                }
            }
        }
    }
}

fn print_usage() {
    println!("Usage: write_tag [OPTIONS] <FILE_OR_DIRECTORY>");
    println!();
    println!("This is a utility for changing tags across MP3 files within a directory.");
    println!();
    println!("OPTIONS:");
    println!("  -t, --title <TITLE>        Change the title frame content");
    println!("  -a, --artist <ARTIST>      Change the artist frame content");
    println!("  -b, --album <ALBUM>        Change the album frame content");
    println!("  -g, --genre <GENRE>        Change the genre frame content");
    println!("  -y, --year <YEAR>          Change the album release year");
    println!("  -c, --comment <COMMENT>    Change the comment frame content");
    println!("  -h, --help                 Show this help message");
    println!();
    println!("ARGUMENTS:");
    println!("  <FILE_OR_DIRECTORY>  Specify file or directory to change tags in (REQUIRED)");
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
            "-t" | "--title" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --title requires a value");
                    process::exit(1);
                }
                options.title = Some(args[i + 1].clone());
                i += 1;
            }
            "-a" | "--artist" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --artist requires a value");
                    process::exit(1);
                }
                options.artist = Some(args[i + 1].clone());
                i += 1;
            }
            "-b" | "--album" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --album requires a value");
                    process::exit(1);
                }
                options.album = Some(args[i + 1].clone());
                i += 1;
            }
            "-g" | "--genre" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --genre requires a value");
                    process::exit(1);
                }
                options.genre = Some(args[i + 1].clone());
                i += 1;
            }
            "-y" | "--year" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --year requires a value");
                    process::exit(1);
                }
                options.year = Some(args[i + 1].clone());
                i += 1;
            }
            "-c" | "--comment" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --comment requires a value");
                    process::exit(1);
                }
                options.comment = Some(args[i + 1].clone());
                i += 1;
            }
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
    
    write_tags(&file_path, &options);
}
