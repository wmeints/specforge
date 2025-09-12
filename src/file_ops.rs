use std::fs;
use std::path::{Path, PathBuf};
use crate::config::ProjectConfig;
use crate::error::{ConfigError, Result};

/// Configuration file name constant
pub const CONFIG_FILE_NAME: &str = ".reforge.json";

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
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    ConfigError::permission_denied(path)
                }
                _ => ConfigError::directory_creation_failed(path, e)
            }
        })?;

        Ok(())
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
}