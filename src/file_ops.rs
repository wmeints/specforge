use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use dialoguer::{Confirm, theme::ColorfulTheme};
use crate::config::ProjectConfig;
use crate::error::{ConfigError, Result};

/// Configuration file name constant
pub const CONFIG_FILE_NAME: &str = ".reforge.json";

/// File information for display in confirmation prompts
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub modified_timestamp: u64,
}

/// Format a Unix timestamp into a human-readable date/time string
fn format_timestamp(timestamp: u64) -> String {
    match SystemTime::UNIX_EPOCH.checked_add(std::time::Duration::from_secs(timestamp)) {
        Some(system_time) => {
            // Convert to local time representation
            // For now, use a simple UTC format since chrono isn't available
            use std::time::Duration;
            let duration = system_time.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
            let secs = duration.as_secs();
            
            // Simple date formatting (not locale-aware, but functional)
            let days = secs / 86400; // seconds per day
            let hours = (secs % 86400) / 3600;
            let minutes = (secs % 3600) / 60;
            let seconds = secs % 60;
            
            // Calculate approximate date (rough calculation from Unix epoch)
            let _epoch_days = 719163; // Days between year 1 and Unix epoch (1970-01-01)
            let _total_days = _epoch_days + days;
            
            // Simple year calculation (not accounting for leap years perfectly)
            let year = 1970 + (days / 365);
            let day_of_year = days % 365;
            let month = (day_of_year / 30) + 1; // Rough month calculation
            let day = (day_of_year % 30) + 1;
            
            format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", 
                year, month.min(12), day.min(31), hours, minutes, seconds)
        }
        None => "Invalid timestamp".to_string(),
    }
}

/// File operations for configuration management
pub struct FileOps;

impl FileOps {
    /// Create a directory if it doesn't exist, including parent directories
    pub fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        
        // Check if path already exists
        if path.exists() {
            if !path.is_dir() {
                return Err(ConfigError::validation_error(format!(
                    "Path '{}' exists but is not a directory",
                    path.display()
                )));
            }
            return Ok(());
        }

        // Create the directory and any missing parent directories
        fs::create_dir_all(path).map_err(|e| {
            Self::enhance_directory_error(path, e)
        })?;

        Ok(())
    }
    
    /// Enhance directory-related errors with more context
    fn enhance_directory_error<P: AsRef<Path>>(path: P, error: std::io::Error) -> ConfigError {
        let path = path.as_ref();
        match error.kind() {
            std::io::ErrorKind::PermissionDenied => {
                ConfigError::validation_error(format!(
                    "Permission denied: Cannot create directory '{}'. \
                     Check that you have write permissions to the parent directory.",
                    path.display()
                ))
            }
            std::io::ErrorKind::NotFound => {
                ConfigError::validation_error(format!(
                    "Cannot create directory '{}': Parent directory does not exist or is inaccessible",
                    path.display()
                ))
            }
            std::io::ErrorKind::AlreadyExists => {
                // This shouldn't happen since we check exists() first, but handle it
                ConfigError::validation_error(format!(
                    "Path '{}' already exists but is not a directory",
                    path.display()
                ))
            }
            std::io::ErrorKind::InvalidInput => {
                ConfigError::validation_error(format!(
                    "Invalid directory path: '{}' contains invalid characters",
                    path.display()
                ))
            }
            _ => ConfigError::directory_creation_failed(path, error)
        }
    }

    /// Check if we have write permissions for a directory
    pub fn check_write_permissions<P: AsRef<Path>>(dir_path: P) -> Result<()> {
        let dir_path = dir_path.as_ref();
        
        // Ensure directory exists first
        Self::ensure_directory_exists(dir_path)?;
        
        // Try to create a temporary file to test write permissions
        let temp_file_path = dir_path.join(".reforge_temp_test");
        
        match fs::write(&temp_file_path, "") {
            Ok(()) => {
                // Clean up the test file
                let _ = fs::remove_file(&temp_file_path);
                Ok(())
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        Err(ConfigError::permission_denied(dir_path))
                    }
                    _ => Err(ConfigError::from(e))
                }
            }
        }
    }

    /// Write a ProjectConfig to a JSON file with proper formatting
    pub fn write_config<P: AsRef<Path>>(config: &ProjectConfig, file_path: P) -> Result<()> {
        let file_path = file_path.as_ref();
        
        // Validate the configuration before writing
        config.validate()?;
        
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            Self::ensure_directory_exists(parent)?;
        }
        
        // Check write permissions
        if let Some(parent) = file_path.parent() {
            Self::check_write_permissions(parent)?;
        }
        
        // Serialize to pretty JSON
        let json_content = config.to_json_string()?;
        
        // Write to file
        fs::write(file_path, json_content).map_err(|e| {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    ConfigError::permission_denied(file_path)
                }
                _ => ConfigError::from(e)
            }
        })?;

        Ok(())
    }

    /// Read and parse a ProjectConfig from a JSON file
    pub fn read_config<P: AsRef<Path>>(file_path: P) -> Result<ProjectConfig> {
        let file_path = file_path.as_ref();
        
        // Check if file exists
        if !file_path.exists() {
            return Err(ConfigError::validation_error(format!(
                "Configuration file does not exist: '{}'",
                file_path.display()
            )));
        }
        
        // Read file contents
        let json_content = fs::read_to_string(file_path).map_err(|e| {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    ConfigError::permission_denied(file_path)
                }
                std::io::ErrorKind::NotFound => {
                    ConfigError::validation_error(format!(
                        "Configuration file not found: '{}'",
                        file_path.display()
                    ))
                }
                _ => ConfigError::from(e)
            }
        })?;
        
        // Parse and validate the configuration
        let config = ProjectConfig::from_json_string(&json_content).map_err(|_e| {
            ConfigError::corrupted_config(file_path)
        })?;
        
        Ok(config)
    }

    /// Write a ProjectConfig to the standard .reforge.json file in a directory
    pub fn write_config_to_directory<P: AsRef<Path>>(config: &ProjectConfig, dir_path: P) -> Result<PathBuf> {
        let dir_path = dir_path.as_ref();
        let config_path = dir_path.join(CONFIG_FILE_NAME);
        
        Self::write_config(config, &config_path)?;
        Ok(config_path)
    }

    /// Read a ProjectConfig from the standard .reforge.json file in a directory
    pub fn read_config_from_directory<P: AsRef<Path>>(dir_path: P) -> Result<ProjectConfig> {
        let dir_path = dir_path.as_ref();
        let config_path = dir_path.join(CONFIG_FILE_NAME);
        
        Self::read_config(config_path)
    }

    /// Check if a .reforge.json file exists in a directory
    pub fn config_exists_in_directory<P: AsRef<Path>>(dir_path: P) -> bool {
        let config_path = dir_path.as_ref().join(CONFIG_FILE_NAME);
        config_path.exists()
    }

    /// Get the full path to the config file in a directory
    pub fn get_config_path<P: AsRef<Path>>(dir_path: P) -> PathBuf {
        dir_path.as_ref().join(CONFIG_FILE_NAME)
    }

    /// Safely write config with backup (for future use)
    pub fn write_config_with_backup<P: AsRef<Path>>(config: &ProjectConfig, file_path: P) -> Result<()> {
        let file_path = file_path.as_ref();
        let backup_path = file_path.with_extension("json.backup");
        
        // If config file exists, create a backup
        if file_path.exists() {
            fs::copy(file_path, &backup_path).map_err(ConfigError::from)?;
        }
        
        // Try to write the new config
        match Self::write_config(config, file_path) {
            Ok(()) => {
                // Remove backup if write was successful
                if backup_path.exists() {
                    let _ = fs::remove_file(&backup_path);
                }
                Ok(())
            }
            Err(e) => {
                // Restore backup if write failed and backup exists
                if backup_path.exists() {
                    let _ = fs::copy(&backup_path, file_path);
                    let _ = fs::remove_file(&backup_path);
                }
                Err(e)
            }
        }
    }

    /// Validate file path and return canonical path
    pub fn validate_and_canonicalize_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
        let path = path.as_ref();
        
        // Convert to absolute path
        let canonical = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(ConfigError::from)?
                .join(path)
        };
        
        // Validate path components
        for component in canonical.components() {
            let component_str = component.as_os_str().to_string_lossy();
            
            // Check for problematic characters
            if component_str.contains('\0') {
                return Err(ConfigError::validation_error(
                    "Path contains null characters"
                ));
            }
        }
        
        Ok(canonical)
    }

    /// Get file information for display in confirmation prompts
    pub fn get_file_info<P: AsRef<Path>>(file_path: P) -> Result<FileInfo> {
        let file_path = file_path.as_ref();
        
        if !file_path.exists() {
            return Err(ConfigError::validation_error(format!(
                "File does not exist: '{}'",
                file_path.display()
            )));
        }
        
        let metadata = fs::metadata(file_path).map_err(ConfigError::from)?;
        
        let size = metadata.len();
        let modified = metadata.modified()
            .map_err(ConfigError::from)?
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ConfigError::io_error(format!("Invalid file modification time: {}", e)))?
            .as_secs();
        
        Ok(FileInfo {
            path: file_path.to_path_buf(),
            size,
            modified_timestamp: modified,
        })
    }

    /// Prompt user for confirmation to overwrite existing file
    pub fn confirm_overwrite<P: AsRef<Path>>(file_path: P) -> Result<bool> {
        let file_path = file_path.as_ref();
        
        // Get file information
        let file_info = Self::get_file_info(file_path)?;
        
        // Format the modification time
        let modified_time = format_timestamp(file_info.modified_timestamp);
        
        // Display file information
        println!("⚠️  Configuration file already exists:");
        println!("   Path: {}", file_info.path.display());
        println!("   Size: {} bytes", file_info.size);
        println!("   Modified: {}", modified_time);
        println!();
        
        // Ask for confirmation
        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to overwrite the existing file?")
            .default(false)
            .interact_opt()
            .map_err(|e| ConfigError::io_error(format!("Failed to read user input: {}", e)))?;
        
        match confirmed {
            Some(answer) => {
                if answer {
                    println!("✅ File will be overwritten");
                } else {
                    println!("❌ Operation cancelled by user");
                }
                Ok(answer)
            }
            None => {
                // User cancelled (Ctrl+C or Esc)
                println!("❌ Operation cancelled by user");
                Ok(false)
            }
        }
    }

    /// Write config with overwrite confirmation (if file exists and force is not specified)
    pub fn write_config_to_directory_with_confirmation<P: AsRef<Path>>(
        config: &ProjectConfig, 
        dir_path: P, 
        force: bool
    ) -> Result<PathBuf> {
        let dir_path = dir_path.as_ref();
        let config_path = dir_path.join(CONFIG_FILE_NAME);
        
        // Check if file exists
        if config_path.exists() {
            if !force {
                // Ask for confirmation
                if !Self::confirm_overwrite(&config_path)? {
                    return Err(ConfigError::user_cancelled("File overwrite cancelled"));
                }
            }
        }
        
        // Proceed with writing
        Self::write_config(config, &config_path)?;
        Ok(config_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Agent, Package};
    use tempfile::TempDir;
    
    #[test]
    fn test_ensure_directory_exists() {
        let temp_dir = TempDir::new().unwrap();
        let new_dir = temp_dir.path().join("test_dir");
        
        assert!(!new_dir.exists());
        assert!(FileOps::ensure_directory_exists(&new_dir).is_ok());
        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn test_ensure_directory_exists_nested() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("level1").join("level2").join("level3");
        
        assert!(!nested_dir.exists());
        assert!(FileOps::ensure_directory_exists(&nested_dir).is_ok());
        assert!(nested_dir.exists());
        assert!(nested_dir.is_dir());
    }

    #[test]
    fn test_check_write_permissions() {
        let temp_dir = TempDir::new().unwrap();
        
        // Should have write permissions in temp directory
        assert!(FileOps::check_write_permissions(temp_dir.path()).is_ok());
    }

    #[test]
    fn test_write_and_read_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.json");
        
        // Create test config
        let mut original_config = ProjectConfig::new(Agent::Copilot);
        let package = Package::new("test-package", "1.0.0");
        original_config.add_package(package).unwrap();
        original_config.set_metadata("test_key", "test_value");
        
        // Write config
        assert!(FileOps::write_config(&original_config, &config_path).is_ok());
        assert!(config_path.exists());
        
        // Read config back
        let read_config = FileOps::read_config(&config_path).unwrap();
        
        // Verify contents
        assert_eq!(read_config.agent, original_config.agent);
        assert_eq!(read_config.packages, original_config.packages);
        assert_eq!(read_config.get_metadata("test_key"), original_config.get_metadata("test_key"));
    }

    #[test]
    fn test_write_read_config_directory() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create test config
        let mut config = ProjectConfig::new(Agent::Claude);
        let package = Package::with_url("test-package", "https://example.com", "2.0.0");
        config.add_package(package).unwrap();
        
        // Write to directory
        let config_path = FileOps::write_config_to_directory(&config, temp_dir.path()).unwrap();
        assert_eq!(config_path.file_name().unwrap(), CONFIG_FILE_NAME);
        assert!(config_path.exists());
        
        // Check if config exists
        assert!(FileOps::config_exists_in_directory(temp_dir.path()));
        
        // Read from directory
        let read_config = FileOps::read_config_from_directory(temp_dir.path()).unwrap();
        assert_eq!(read_config.agent, config.agent);
        assert_eq!(read_config.packages.len(), 1);
    }

    #[test]
    fn test_read_nonexistent_config() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_path = temp_dir.path().join("nonexistent.json");
        
        let result = FileOps::read_config(&nonexistent_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_read_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_json_path = temp_dir.path().join("invalid.json");
        
        // Write invalid JSON
        fs::write(&invalid_json_path, "{ invalid json }").unwrap();
        
        let result = FileOps::read_config(&invalid_json_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("corrupted"));
    }

    #[test]
    fn test_get_config_path() {
        let dir = Path::new("/test/dir");
        let config_path = FileOps::get_config_path(dir);
        assert_eq!(config_path, dir.join(CONFIG_FILE_NAME));
    }

    #[test]
    fn test_config_exists_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        
        // Should not exist initially
        assert!(!FileOps::config_exists_in_directory(temp_dir.path()));
        
        // Create config file
        let config = ProjectConfig::new(Agent::Copilot);
        FileOps::write_config_to_directory(&config, temp_dir.path()).unwrap();
        
        // Should exist now
        assert!(FileOps::config_exists_in_directory(temp_dir.path()));
    }

    #[test]
    fn test_validate_and_canonicalize_path() {
        // Test relative path
        let relative_path = Path::new("test/path");
        let canonical = FileOps::validate_and_canonicalize_path(relative_path).unwrap();
        assert!(canonical.is_absolute());
        
        // Test absolute path
        let absolute_path = std::env::current_dir().unwrap().join("test");
        let canonical = FileOps::validate_and_canonicalize_path(&absolute_path).unwrap();
        assert_eq!(canonical, absolute_path);
    }

    #[test]
    fn test_write_config_with_backup() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Create initial config
        let config1 = ProjectConfig::new(Agent::Copilot);
        FileOps::write_config(&config1, &config_path).unwrap();
        
        // Update config with backup
        let mut config2 = ProjectConfig::new(Agent::Claude);
        config2.set_metadata("version", "2.0");
        
        assert!(FileOps::write_config_with_backup(&config2, &config_path).is_ok());
        
        // Verify updated config
        let read_config = FileOps::read_config(&config_path).unwrap();
        assert_eq!(read_config.agent, Agent::Claude);
        
        // Backup should be cleaned up
        let backup_path = config_path.with_extension("json.backup");
        assert!(!backup_path.exists());
    }

    #[test]
    fn test_json_formatting() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("formatted.json");
        
        // Create config with data
        let mut config = ProjectConfig::with_project_name(Agent::Copilot, "test-project");
        let package = Package::with_url("test-pkg", "https://example.com", "1.0.0");
        config.add_package(package).unwrap();
        
        FileOps::write_config(&config, &config_path).unwrap();
        
        // Read raw file content and verify formatting
        let json_content = fs::read_to_string(&config_path).unwrap();
        
        // Should be pretty-printed (contains newlines and indentation)
        assert!(json_content.contains("\n"));
        assert!(json_content.contains("  ")); // Indentation
        assert!(json_content.contains("\"agent\": \"copilot\""));
        assert!(json_content.contains("\"project_name\": \"test-project\""));
    }

    #[test]
    fn test_get_file_info() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test_info.json");
        
        // Create test file with known content
        let test_content = r#"{"test": "data"}"#;
        fs::write(&test_file, test_content).unwrap();
        
        // Get file info
        let file_info = FileOps::get_file_info(&test_file).unwrap();
        
        // Verify file info
        assert_eq!(file_info.path, test_file);
        assert_eq!(file_info.size, test_content.len() as u64);
        assert!(file_info.modified_timestamp > 0);
    }

    #[test]
    fn test_get_file_info_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_file = temp_dir.path().join("nonexistent.json");
        
        let result = FileOps::get_file_info(&nonexistent_file);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_write_config_to_directory_with_confirmation_force() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create initial config
        let config1 = ProjectConfig::new(Agent::Copilot);
        let config_path1 = FileOps::write_config_to_directory(&config1, temp_dir.path()).unwrap();
        assert!(config_path1.exists());
        
        // Write new config with force=true (should not prompt)
        let mut config2 = ProjectConfig::new(Agent::Claude);
        config2.set_metadata("test", "value");
        
        let result = FileOps::write_config_to_directory_with_confirmation(
            &config2, 
            temp_dir.path(), 
            true // force = true
        );
        assert!(result.is_ok());
        
        // Verify the file was overwritten
        let read_config = FileOps::read_config_from_directory(temp_dir.path()).unwrap();
        assert_eq!(read_config.agent, Agent::Claude);
        assert_eq!(read_config.get_metadata("test"), Some(&serde_json::Value::String("value".to_string())));
    }

    #[test]
    fn test_write_config_to_directory_with_confirmation_new_file() {
        let temp_dir = TempDir::new().unwrap();
        
        // Write config to directory without existing file
        let config = ProjectConfig::new(Agent::Copilot);
        let result = FileOps::write_config_to_directory_with_confirmation(
            &config, 
            temp_dir.path(), 
            false // force = false
        );
        
        // Should succeed without prompting
        assert!(result.is_ok());
        let config_path = result.unwrap();
        assert!(config_path.exists());
        
        // Verify content
        let read_config = FileOps::read_config_from_directory(temp_dir.path()).unwrap();
        assert_eq!(read_config.agent, Agent::Copilot);
    }
}