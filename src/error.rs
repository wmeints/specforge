use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ValidationError(String),
    InvalidAgent(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(err) => {
                write!(f, "File system error: {}", err)
            }
            ConfigError::JsonError(err) => {
                write!(f, "JSON parsing error: {}", err)
            }
            ConfigError::ValidationError(msg) => {
                write!(f, "Configuration validation error: {}", msg)
            }
            ConfigError::InvalidAgent(agent) => {
                write!(f, "Invalid agent '{}'. Supported agents: copilot, claude", agent)
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IoError(err) => Some(err),
            ConfigError::JsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::JsonError(err)
    }
}

pub type Result<T> = std::result::Result<T, ConfigError>;