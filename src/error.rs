use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    /// File system operation failed
    IoError(std::io::Error),
    /// JSON serialization/deserialization failed
    JsonError(serde_json::Error),
    /// Configuration validation failed
    ValidationError(String),
    /// Invalid agent specified
    InvalidAgent(String),
    /// File already exists and user declined overwrite
    FileExists(PathBuf),
    /// Permission denied for file or directory operation
    PermissionDenied(PathBuf),
    /// Directory creation failed
    DirectoryCreationFailed(PathBuf, std::io::Error),
    /// Configuration file is corrupted or invalid
    CorruptedConfig(PathBuf),
    /// Required field missing from configuration
    MissingRequiredField(String),
    /// Invalid package configuration
    InvalidPackage(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(err) => {
                write!(f, "File system operation failed: {}\n\nTry checking file permissions and disk space.", err)
            }
            ConfigError::JsonError(err) => {
                write!(f, "Failed to parse JSON configuration: {}\n\nEnsure the .reforge.json file contains valid JSON syntax.", err)
            }
            ConfigError::ValidationError(msg) => {
                write!(f, "Configuration validation failed: {}\n\nPlease check your configuration file format.", msg)
            }
            ConfigError::InvalidAgent(agent) => {
                write!(f, "Invalid agent '{}' specified.\n\nSupported agents are: 'copilot' or 'claude'.\nExample: reforge init --agent copilot", agent)
            }
            ConfigError::FileExists(path) => {
                write!(f, "Configuration file already exists at: {}\n\nUse 'reforge init --force' to overwrite or choose a different directory.", path.display())
            }
            ConfigError::PermissionDenied(path) => {
                write!(f, "Permission denied accessing: {}\n\nCheck that you have read/write permissions for this location.", path.display())
            }
            ConfigError::DirectoryCreationFailed(path, err) => {
                write!(f, "Failed to create directory '{}': {}\n\nEnsure the parent directory exists and you have write permissions.", path.display(), err)
            }
            ConfigError::CorruptedConfig(path) => {
                write!(f, "Configuration file is corrupted or invalid: {}\n\nDelete the file and run 'reforge init' to recreate it.", path.display())
            }
            ConfigError::MissingRequiredField(field) => {
                write!(f, "Required field '{}' is missing from configuration.\n\nDelete the .reforge.json file and run 'reforge init' to recreate it.", field)
            }
            ConfigError::InvalidPackage(msg) => {
                write!(f, "Invalid package configuration: {}\n\nCheck the packages array in your .reforge.json file.", msg)
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IoError(err) => Some(err),
            ConfigError::JsonError(err) => Some(err),
            ConfigError::DirectoryCreationFailed(_, err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                // We'll need the path context from the caller for this
                ConfigError::IoError(err)
            }
            _ => ConfigError::IoError(err),
        }
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::JsonError(err)
    }
}

impl ConfigError {
    /// Create a permission denied error with path context
    pub fn permission_denied<P: Into<PathBuf>>(path: P) -> Self {
        ConfigError::PermissionDenied(path.into())
    }

    /// Create a file exists error with path context
    pub fn file_exists<P: Into<PathBuf>>(path: P) -> Self {
        ConfigError::FileExists(path.into())
    }

    /// Create a directory creation failed error
    pub fn directory_creation_failed<P: Into<PathBuf>>(path: P, err: std::io::Error) -> Self {
        ConfigError::DirectoryCreationFailed(path.into(), err)
    }

    /// Create a corrupted config error
    pub fn corrupted_config<P: Into<PathBuf>>(path: P) -> Self {
        ConfigError::CorruptedConfig(path.into())
    }

    /// Create a missing required field error
    pub fn missing_required_field<S: Into<String>>(field: S) -> Self {
        ConfigError::MissingRequiredField(field.into())
    }

    /// Create an invalid agent error
    pub fn invalid_agent<S: Into<String>>(agent: S) -> Self {
        ConfigError::InvalidAgent(agent.into())
    }

    /// Create an invalid package error
    pub fn invalid_package<S: Into<String>>(msg: S) -> Self {
        ConfigError::InvalidPackage(msg.into())
    }

    /// Create a validation error
    pub fn validation_error<S: Into<String>>(msg: S) -> Self {
        ConfigError::ValidationError(msg.into())
    }
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_agent_error() {
        let error = ConfigError::invalid_agent("invalid");
        let msg = error.to_string();
        assert!(msg.contains("Invalid agent 'invalid'"));
        assert!(msg.contains("copilot"));
        assert!(msg.contains("claude"));
    }

    #[test]
    fn test_permission_denied_error() {
        let error = ConfigError::permission_denied("/test/path");
        let msg = error.to_string();
        assert!(msg.contains("Permission denied"));
        assert!(msg.contains("/test/path"));
        assert!(msg.contains("read/write permissions"));
    }

    #[test]
    fn test_file_exists_error() {
        let error = ConfigError::file_exists("/test/.reforge.json");
        let msg = error.to_string();
        assert!(msg.contains("Configuration file already exists"));
        assert!(msg.contains("/test/.reforge.json"));
        assert!(msg.contains("--force"));
    }

    #[test]
    fn test_missing_required_field_error() {
        let error = ConfigError::missing_required_field("agent");
        let msg = error.to_string();
        assert!(msg.contains("Required field 'agent'"));
        assert!(msg.contains("reforge init"));
    }

    #[test]
    fn test_error_trait_implementation() {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        let config_error = ConfigError::from(io_error);
        
        // Should implement Error trait
        let _: &dyn std::error::Error = &config_error;
    }

    #[test]
    fn test_json_error_conversion() {
        let json_str = "invalid json {";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let config_error = ConfigError::from(json_error);
        
        match config_error {
            ConfigError::JsonError(_) => (),
            _ => panic!("Should be JsonError variant"),
        }
    }
}