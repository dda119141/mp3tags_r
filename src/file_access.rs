use std::fs::File;
use std::path::Path;
use crate::Result;

/// Strategy trait for different file access patterns
pub trait FileAccessStrategy: Send + Sync {
    /// Open a file for reading
    fn open_for_read(&self, path: &Path) -> Result<File>;
    
    /// Open a file for writing (creates or truncates)
    fn open_for_write(&self, path: &Path) -> Result<File>;
    
    /// Open a file for reading and writing
    fn open_for_read_write(&self, path: &Path) -> Result<File>;
    
    /// Check if a file exists
    fn exists(&self, path: &Path) -> bool;
    
    /// Get file metadata
    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata>;
}


/// Standard file access strategy using the filesystem directly
pub struct StandardFileAccess;

impl FileAccessStrategy for StandardFileAccess {
    fn open_for_read(&self, path: &Path) -> Result<File> {
        Ok(File::open(path)?)
    }
    
    fn open_for_write(&self, path: &Path) -> Result<File> {
        Ok(File::create(path)?)
    }
    
    fn open_for_read_write(&self, path: &Path) -> Result<File> {
        Ok(std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?)
    }
    
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
    
    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata> {
        Ok(std::fs::metadata(path)?)
    }
}


/// Factory for creating file access strategies
pub struct FileAccessFactory;

impl FileAccessFactory {
    /// Create a standard file access strategy
    pub fn create_standard() -> Box<dyn FileAccessStrategy> {
        Box::new(StandardFileAccess)
    }
    
    /// Create the default strategy (standard for now)
    pub fn create_default() -> Box<dyn FileAccessStrategy> {
        Self::create_standard()
    }
}

/// File manager that uses a strategy for file operations
pub struct FileManager {
    strategy: Box<dyn FileAccessStrategy>,
}

impl FileManager {
    /// Create a new file manager with the given strategy
    pub fn new(strategy: Box<dyn FileAccessStrategy>) -> Self {
        Self { strategy }
    }
    
    /// Create a file manager with the default strategy
    pub fn with_default_strategy() -> Self {
        Self::new(FileAccessFactory::create_default())
    }
    
    /// Open a file for reading
    pub fn open_for_read(&self, path: &Path) -> Result<File> {
        self.strategy.open_for_read(path)
    }
    
    /// Open a file for writing
    pub fn open_for_write(&self, path: &Path) -> Result<File> {
        self.strategy.open_for_write(path)
    }
    
    /// Open a file for reading and writing
    pub fn open_for_read_write(&self, path: &Path) -> Result<File> {
        self.strategy.open_for_read_write(path)
    }
    
    /// Check if a file exists
    pub fn exists(&self, path: &Path) -> bool {
        self.strategy.exists(path)
    }
    
    /// Get file metadata
    pub fn metadata(&self, path: &Path) -> Result<std::fs::Metadata> {
        self.strategy.metadata(path)
    }
    
    /// Validate that a path exists and is a readable file
    pub fn validate_file_path(&self, path: &Path) -> Result<()> {
        if !self.exists(path) {
            return Err(crate::Error::FileError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display())
            )));
        }
        
        let metadata = self.metadata(path)?;
        if !metadata.is_file() {
            return Err(crate::Error::FileError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Path is not a file: {}", path.display())
            )));
        }
        
        Ok(())
    }
}

/// Global default file manager instance
static DEFAULT_FILE_MANAGER: std::sync::OnceLock<FileManager> = std::sync::OnceLock::new();

/// Get the default file manager instance
pub fn default_file_manager() -> &'static FileManager {
    DEFAULT_FILE_MANAGER.get_or_init(|| FileManager::with_default_strategy())
}
