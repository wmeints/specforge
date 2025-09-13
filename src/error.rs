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
    /// User cancelled operation
    UserCancelled(String),
    /// Context-aware error with detailed information
    ContextualError {
        operation: String,
        cause: Box<ConfigError>,
        context: String,
    },
    /// Network or connectivity related error
    NetworkError(String),
    /// Insufficient disk space
    DiskSpaceError(PathBuf),
    /// File or directory not found
    NotFound(PathBuf),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(err) => {
                let (suggestion, _debug_info) = Self::analyze_io_error(err);
                write!(f, "File system operation failed: {}\n\n{}\n\nDebug info: {} (ErrorKind: {:?})",
                    err, suggestion, err, err.kind())
            }
            ConfigError::JsonError(err) => {
                write!(f, "Failed to parse JSON configuration: {}\n\nEnsure the .reforge.json file contains valid JSON syntax.\nTip: You can validate JSON online or use 'cat .reforge.json | jq .' to check formatting.\n\nDebug info: Line {}, Column {}",
                    err,
                    err.line(),
                    err.column())
            }
            ConfigError::ValidationError(msg) => {
                write!(f, "Configuration validation failed: {}\n\nPlease check your configuration file format and ensure all required fields are present.\nFor reference, run 'reforge init' to see the expected format.", msg)
            }
            ConfigError::InvalidAgent(agent) => {
                write!(f, "Invalid agent '{}' specified.\n\nSupported agents are:\n  • 'copilot' - GitHub Copilot integration\n  • 'claude' - Anthropic Claude integration\n\nExamples:\n  reforge init --agent copilot\n  reforge init --agent claude", agent)
            }
            ConfigError::FileExists(path) => {
                write!(f, "Configuration file already exists at: {}\n\nOptions:\n  • Use 'reforge init --force' to overwrite\n  • Choose a different directory with '--output-directory <path>'\n  • Remove the existing file manually: rm {}",
                    path.display(), path.display())
            }
            ConfigError::PermissionDenied(path) => {
                write!(f, "Permission denied accessing: {}\n\nTroubleshooting steps:\n  1. Check file/directory permissions: ls -la {}\n  2. Ensure you own the directory or have write access\n  3. Try running with appropriate permissions\n  4. Choose a different directory you have write access to",
                    path.display(),
                    path.parent().unwrap_or(path).display())
            }
            ConfigError::DirectoryCreationFailed(path, err) => {
                write!(f, "Failed to create directory '{}': {}\n\nTroubleshooting:\n  • Ensure parent directories exist and are writable\n  • Check available disk space: df -h\n  • Verify path doesn't conflict with existing files\n  • Try a different output directory",
                    path.display(), err)
            }
            ConfigError::CorruptedConfig(path) => {
                write!(f, "Configuration file is corrupted or invalid: {}\n\nRecovery options:\n  1. Backup the current file: cp {} {}.backup\n  2. Delete the corrupted file: rm {}\n  3. Recreate with: reforge init\n  4. Restore from backup if needed",
                    path.display(), path.display(), path.display(), path.display())
            }
            ConfigError::MissingRequiredField(field) => {
                write!(f, "Required field '{}' is missing from configuration.\n\nQuick fix:\n  1. Backup current config: cp .reforge.json .reforge.json.backup\n  2. Recreate config: reforge init\n  3. Merge custom settings from backup if needed", field)
            }
            ConfigError::InvalidPackage(msg) => {
                write!(f, "Invalid package configuration: {}\n\nPackage requirements:\n  • ID must be non-empty and contain no whitespace\n  • Version must follow semantic versioning (e.g., '1.0.0')\n  • URL (if provided) must start with 'http://' or 'https://'\n\nCheck the packages array in your .reforge.json file.", msg)
            }
            ConfigError::UserCancelled(msg) => {
                write!(f, "Operation cancelled: {}\n\nYou can restart the operation at any time.", msg)
            }
            ConfigError::ContextualError { operation, cause, context } => {
                write!(f, "Error during {}: {}\n\nContext: {}\n\nUnderlying cause: {}",
                    operation, cause, context, cause)
            }
            ConfigError::NetworkError(msg) => {
                write!(f, "Network operation failed: {}\n\nTroubleshooting:\n  • Check your internet connection\n  • Verify firewall/proxy settings\n  • Try again in a few moments\n  • Check if the remote service is available", msg)
            }
            ConfigError::DiskSpaceError(path) => {
                write!(f, "Insufficient disk space for operation in: {}\n\nSolutions:\n  • Free up disk space: check 'df -h' for usage\n  • Choose a different directory with more space\n  • Clean up temporary files\n  • Remove unused files", path.display())
            }
            ConfigError::NotFound(path) => {
                write!(f, "File or directory not found: {}\n\nVerification:\n  • Check if the path exists: ls -la {}\n  • Verify correct spelling and case sensitivity\n  • Ensure you're in the correct working directory\n  • Check if the file was moved or deleted",
                    path.display(), path.display())
            }
        }
    }
}

impl ConfigError {
    /// Analyze IO errors to provide better context and suggestions
    fn analyze_io_error(err: &std::io::Error) -> (String, String) {
        let suggestion = match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                "Check file/directory permissions and ensure you have write access.\nTry: chmod 755 <directory> or choose a different location.".to_string()
            }
            std::io::ErrorKind::NotFound => {
                "The specified file or directory does not exist.\nVerify the path and ensure parent directories exist.".to_string()
            }
            std::io::ErrorKind::AlreadyExists => {
                "A file or directory with this name already exists.\nUse --force to overwrite or choose a different name/location.".to_string()
            }
            std::io::ErrorKind::InvalidInput => {
                "Invalid path or filename provided.\nCheck for special characters or invalid path format.".to_string()
            }
            std::io::ErrorKind::UnexpectedEof => {
                "File appears to be truncated or corrupted.\nTry recreating the file or restoring from backup.".to_string()
            }
            std::io::ErrorKind::WriteZero => {
                "Could not write any data (possibly disk full).\nCheck available disk space with 'df -h'.".to_string()
            }
            std::io::ErrorKind::Interrupted => {
                "Operation was interrupted.\nThis is usually safe to retry.".to_string()
            }
            _ => {
                "File system operation failed.\nCheck permissions, disk space, and path validity.".to_string()
            }
        };

        let debug_info = format!("OS Error: {} (Kind: {:?})", err.raw_os_error().unwrap_or(-1), err.kind());
        (suggestion, debug_info)
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IoError(err) => Some(err),
            ConfigError::JsonError(err) => Some(err),
            ConfigError::DirectoryCreationFailed(_, err) => Some(err),
            ConfigError::ContextualError { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        // Enhanced error categorization
        match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                // Note: Callers should use permission_denied() with path context when possible
                ConfigError::IoError(err)
            }
            std::io::ErrorKind::NotFound => {
                ConfigError::IoError(err)
            }
            std::io::ErrorKind::WriteZero => {
                // Likely disk space issue, but we need path context from caller
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

    /// Create a user cancelled error
    pub fn user_cancelled<S: Into<String>>(msg: S) -> Self {
        ConfigError::UserCancelled(msg.into())
    }

    /// Create an I/O error with message
    pub fn io_error<S: Into<String>>(msg: S) -> Self {
        ConfigError::IoError(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
    }

    /// Create a contextual error with operation details
    pub fn with_context<S1: Into<String>, S2: Into<String>>(
        operation: S1,
        cause: ConfigError,
        context: S2,
    ) -> Self {
        ConfigError::ContextualError {
            operation: operation.into(),
            cause: Box::new(cause),
            context: context.into(),
        }
    }

    /// Create a network error
    pub fn network_error<S: Into<String>>(msg: S) -> Self {
        ConfigError::NetworkError(msg.into())
    }

    /// Create a disk space error
    pub fn disk_space_error<P: Into<PathBuf>>(path: P) -> Self {
        ConfigError::DiskSpaceError(path.into())
    }

    /// Create a not found error
    pub fn not_found<P: Into<PathBuf>>(path: P) -> Self {
        ConfigError::NotFound(path.into())
    }

    /// Add context to an existing error
    pub fn add_context<S1: Into<String>, S2: Into<String>>(
        self,
        operation: S1,
        context: S2,
    ) -> Self {
        ConfigError::ContextualError {
            operation: operation.into(),
            cause: Box::new(self),
            context: context.into(),
        }
    }

    /// Check if this error should be retried
    pub fn is_retryable(&self) -> bool {
        match self {
            ConfigError::IoError(err) => matches!(
                err.kind(),
                std::io::ErrorKind::Interrupted | std::io::ErrorKind::TimedOut
            ),
            ConfigError::NetworkError(_) => true,
            ConfigError::ContextualError { cause, .. } => cause.is_retryable(),
            _ => false,
        }
    }

    /// Get the exit code for this error type
    pub fn exit_code(&self) -> i32 {
        match self {
            ConfigError::PermissionDenied(_) => 13,  // Permission denied
            ConfigError::FileExists(_) => 17,        // File exists
            ConfigError::InvalidAgent(_) => 22,      // Invalid argument
            ConfigError::ValidationError(_) => 22,   // Invalid argument
            ConfigError::MissingRequiredField(_) => 22, // Invalid argument
            ConfigError::InvalidPackage(_) => 22,    // Invalid argument
            ConfigError::CorruptedConfig(_) => 74,   // IO error
            ConfigError::DirectoryCreationFailed(_, _) => 73, // Can't create
            ConfigError::IoError(_) => 74,           // IO error
            ConfigError::JsonError(_) => 65,         // Data format error
            ConfigError::UserCancelled(_) => 1,      // User cancelled operation
            ConfigError::ContextualError { cause, .. } => cause.exit_code(),
            ConfigError::NetworkError(_) => 69,      // Service unavailable
            ConfigError::DiskSpaceError(_) => 28,    // No space left on device
            ConfigError::NotFound(_) => 2,           // No such file or directory
        }
    }

    /// Log this error appropriately without exposing sensitive information
    pub fn log_securely(&self) {
        match self {
            ConfigError::PermissionDenied(path) => {
                eprintln!("DEBUG: Permission denied for path (length: {} chars)", path.as_os_str().len());
            }
            ConfigError::DirectoryCreationFailed(path, err) => {
                eprintln!("DEBUG: Directory creation failed - OS error: {:?}, path length: {}",
                    err.kind(), path.as_os_str().len());
            }
            ConfigError::IoError(err) => {
                eprintln!("DEBUG: IO error - kind: {:?}, OS error: {:?}",
                    err.kind(), err.raw_os_error());
            }
            ConfigError::JsonError(err) => {
                eprintln!("DEBUG: JSON parsing error at line {}, column {}",
                    err.line(), err.column());
            }
            ConfigError::ContextualError { operation, cause, .. } => {
                eprintln!("DEBUG: Error in operation '{}', underlying cause:", operation);
                cause.log_securely();
            }
            _ => {
                eprintln!("DEBUG: Error occurred: {}", std::any::type_name::<Self>());
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

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
        assert!(msg.contains("write access"));  // Updated to match new message format
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

    #[test]
    fn test_contextual_error_creation() {
        let base_error = ConfigError::permission_denied("/test/path");
        let contextual_error = ConfigError::with_context(
            "file creation",
            base_error,
            "Attempting to create configuration file"
        );

        let msg = contextual_error.to_string();
        assert!(msg.contains("Error during file creation"));
        assert!(msg.contains("Context: Attempting to create configuration file"));
        assert!(msg.contains("Permission denied"));
    }

    #[test]
    fn test_add_context_method() {
        let base_error = ConfigError::invalid_agent("unknown");
        let contextual_error = base_error.add_context(
            "initialization",
            "Setting up project with specified agent"
        );

        match contextual_error {
            ConfigError::ContextualError { operation, context, .. } => {
                assert_eq!(operation, "initialization");
                assert_eq!(context, "Setting up project with specified agent");
            }
            _ => panic!("Should be ContextualError variant"),
        }
    }

    #[test]
    fn test_exit_codes() {
        // Test Unix convention exit codes
        assert_eq!(ConfigError::permission_denied("/test").exit_code(), 13);
        assert_eq!(ConfigError::file_exists("/test").exit_code(), 17);
        assert_eq!(ConfigError::invalid_agent("test").exit_code(), 22);
        assert_eq!(ConfigError::not_found("/test").exit_code(), 2);
        assert_eq!(ConfigError::disk_space_error("/test").exit_code(), 28);
        assert_eq!(ConfigError::network_error("test").exit_code(), 69);
        assert_eq!(ConfigError::user_cancelled("test").exit_code(), 1);
    }

    #[test]
    fn test_contextual_error_exit_code() {
        let base_error = ConfigError::permission_denied("/test");
        let expected_code = base_error.exit_code();

        let contextual_error = base_error.add_context("test operation", "test context");
        assert_eq!(contextual_error.exit_code(), expected_code);
    }

    #[test]
    fn test_retryable_errors() {
        // Network errors should be retryable
        assert!(ConfigError::network_error("connection failed").is_retryable());

        // Interrupted IO operations should be retryable
        let interrupted_io = std::io::Error::new(std::io::ErrorKind::Interrupted, "interrupted");
        assert!(ConfigError::IoError(interrupted_io).is_retryable());

        // Permission errors should not be retryable
        assert!(!ConfigError::permission_denied("/test").is_retryable());

        // Invalid agent should not be retryable
        assert!(!ConfigError::invalid_agent("invalid").is_retryable());
    }

    #[test]
    fn test_contextual_error_retryable() {
        let retryable_base = ConfigError::network_error("connection failed");
        assert!(retryable_base.is_retryable());

        let contextual_retryable = retryable_base.add_context("download", "Downloading templates");
        assert!(contextual_retryable.is_retryable());

        let non_retryable_base = ConfigError::invalid_agent("test");
        assert!(!non_retryable_base.is_retryable());

        let contextual_non_retryable = non_retryable_base.add_context("validation", "Checking agent");
        assert!(!contextual_non_retryable.is_retryable());
    }

    #[test]
    fn test_enhanced_error_messages() {
        // Test that enhanced error messages contain helpful information
        let json_error = ConfigError::JsonError(
            serde_json::from_str::<serde_json::Value>("invalid json {").unwrap_err()
        );
        let msg = json_error.to_string();
        assert!(msg.contains("jq"));  // Suggests jq for validation
        assert!(msg.contains("Line"));  // Contains line number info

        let agent_error = ConfigError::invalid_agent("invalid");
        let agent_msg = agent_error.to_string();
        assert!(agent_msg.contains("•"));  // Contains bullet points for readability
        assert!(agent_msg.contains("Examples:"));  // Contains examples

        let permission_error = ConfigError::permission_denied("/test/path");
        let perm_msg = permission_error.to_string();
        assert!(perm_msg.contains("ls -la"));  // Contains debugging commands
        assert!(perm_msg.contains("Troubleshooting"));  // Contains troubleshooting section
    }

    #[test]
    fn test_io_error_analysis() {
        // Test different IO error kinds get appropriate suggestions
        let permission_io = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        let config_error = ConfigError::IoError(permission_io);
        let msg = config_error.to_string();
        assert!(msg.contains("chmod"));
        assert!(msg.contains("write access"));

        let not_found_io = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let config_error = ConfigError::IoError(not_found_io);
        let msg = config_error.to_string();
        assert!(msg.contains("does not exist"));
        assert!(msg.contains("parent directories"));
    }

    #[test]
    fn test_secure_logging() {
        // Test that secure logging doesn't expose sensitive information
        let error = ConfigError::permission_denied("/very/long/sensitive/path/to/secret/file.txt");

        // This test captures stderr to verify logging doesn't expose paths
        // In a real scenario, we'd want to make sure paths aren't logged in full

        // For now, just verify the method exists and doesn't panic
        error.log_securely();
    }

    #[test]
    fn test_new_error_types() {
        // Test new error types
        let network_error = ConfigError::network_error("Connection timeout");
        assert!(network_error.to_string().contains("internet connection"));

        let disk_error = ConfigError::disk_space_error("/tmp");
        assert!(disk_error.to_string().contains("disk space"));
        assert!(disk_error.to_string().contains("df -h"));

        let not_found_error = ConfigError::not_found("/missing/file");
        assert!(not_found_error.to_string().contains("not found"));
        assert!(not_found_error.to_string().contains("ls -la"));
    }

    #[test]
    fn test_error_source_chain() {
        // Test that error source chain works correctly
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        let config_error = ConfigError::IoError(io_error);
        assert!(config_error.source().is_some());

        let contextual_error = config_error.add_context("test", "context");
        assert!(contextual_error.source().is_some());

        // Source chain should point to the original error
        let source = contextual_error.source().unwrap();
        let downcast_result = source.downcast_ref::<ConfigError>();
        assert!(downcast_result.is_some());
    }
}