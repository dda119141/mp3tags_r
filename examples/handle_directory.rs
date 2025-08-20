use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Default)]
struct DirectoryOptions {
    empty: bool,
    remove: bool,
}

impl DirectoryOptions {
    fn is_empty(&self) -> bool {
        !(self.empty || self.remove)
    }
}

/// Delete all files in a directory and then the directory itself
fn delete_files(dir_path: &Path) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            fs::remove_file(&path)?;
        }
    }
    
    fs::remove_dir(dir_path)?;
    Ok(())
}

/// Check if a directory should be deleted based on criteria:
/// - Must be a directory
/// - Should not contain subdirectories
/// - Should not contain files larger than 1MB
fn find_directory_to_delete(dir_path: &Path) -> Result<bool, std::io::Error> {
    if !dir_path.is_dir() {
        return Ok(false);
    }
    
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            return Ok(false);
        } else if path.is_file() {
            let file_size = fs::metadata(&path)?.len();
            if file_size > 1024 * 1024 {  // 1MB
                return Ok(false);
            }
        }
    }
    
    Ok(true)
}

/// Remove obsolete directories based on the specified criteria
fn remove_obsolete_directory(dir_path: &Path, options: &DirectoryOptions) -> Result<(), std::io::Error> {
    let mut directories_to_delete = Vec::new();
    
    // Collect directories that match deletion criteria
    fn collect_directories(path: &Path, dirs: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                
                if entry_path.is_dir() {
                    if find_directory_to_delete(&entry_path)? {
                        println!("Path to Delete: {}", entry_path.display());
                        dirs.push(entry_path.clone());
                    }
                    // Recursively check subdirectories
                    collect_directories(&entry_path, dirs)?;
                }
            }
        }
        Ok(())
    }
    
    collect_directories(dir_path, &mut directories_to_delete)?;
    
    if directories_to_delete.is_empty() {
        println!("No directories found matching deletion criteria.");
        return Ok(());
    }
    
    if options.remove {
        for dir in directories_to_delete {
            match delete_files(&dir) {
                Ok(_) => println!("Successfully deleted: {}", dir.display()),
                Err(e) => eprintln!("Failed to delete {}: {}", dir.display(), e),
            }
        }
    } else {
        println!("Found {} directories that would be deleted (use --remove to actually delete them):", 
                directories_to_delete.len());
        for dir in directories_to_delete {
            println!("  {}", dir.display());
        }
    }
    
    Ok(())
}

/// Process the specified directory path
fn remove_paths(directory: &str, options: &DirectoryOptions) -> Result<(), std::io::Error> {
    let current_file_path = fs::canonicalize(directory)?;
    
    if !current_file_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Path: {} does not exist", directory)
        ));
    }
    
    if !current_file_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path: {} is not a directory", directory)
        ));
    }
    
    remove_obsolete_directory(&current_file_path, options)?;
    Ok(())
}

fn print_usage() {
    println!("handle_directory - Utility for managing directories with small files");
    println!();
    println!("USAGE:");
    println!("    handle_directory [OPTIONS] --directory <DIR>");
    println!();
    println!("OPTIONS:");
    println!("    -d, --directory <DIR>    Specify directory to process (REQUIRED)");
    println!("    -t, --empty             Show empty directories");
    println!("    -r, --remove            Actually remove the directories (default: dry run)");
    println!("    -h, --help              Print this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    This utility finds directories that contain only small files (< 1MB)");
    println!("    and no subdirectories. Use --remove to actually delete them.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    let mut options = DirectoryOptions::default();
    let mut directory = String::new();
    let mut show_help = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--directory" => {
                if i + 1 < args.len() {
                    directory = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --directory requires a value");
                    process::exit(1);
                }
            }
            "-t" | "--empty" => {
                options.empty = true;
            }
            "-r" | "--remove" => {
                options.remove = true;
            }
            "-h" | "--help" => {
                show_help = true;
            }
            _ => {
                eprintln!("Error: Unknown argument '{}'", args[i]);
                print_usage();
                process::exit(1);
            }
        }
        i += 1;
    }
    
    if show_help {
        print_usage();
        return;
    }
    
    if directory.is_empty() || options.is_empty() {
        eprintln!("Error: Directory is required and at least one option must be specified");
        print_usage();
        process::exit(1);
    }
    
    if let Err(e) = remove_paths(&directory, &options) {
        eprintln!("Error processing directory: {}", e);
        process::exit(1);
    }
    
    println!("Directory processing completed successfully.");
}
