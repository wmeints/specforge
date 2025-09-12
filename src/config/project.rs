use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::error::{ConfigError, Result};

/// Represents the different types of AI agents supported by Reforge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Agent {
    /// GitHub Copilot
    Copilot,
    /// Anthropic Claude
    Claude,
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Agent::Copilot => write!(f, "copilot"),
            Agent::Claude => write!(f, "claude"),
        }
    }
}

impl FromStr for Agent {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "copilot" => Ok(Agent::Copilot),
            "claude" => Ok(Agent::Claude),
            _ => Err(ConfigError::invalid_agent(s)),
        }
    }
}

impl Agent {
    /// Returns all supported agent types
    pub fn all() -> Vec<Agent> {
        vec![Agent::Copilot, Agent::Claude]
    }

    /// Returns all supported agent names as strings
    pub fn all_names() -> Vec<&'static str> {
        vec!["copilot", "claude"]
    }

    /// Returns a human-readable description of the agent
    pub fn description(&self) -> &'static str {
        match self {
            Agent::Copilot => "GitHub Copilot - AI pair programmer integrated with your editor",
            Agent::Claude => "Anthropic Claude - Advanced AI assistant for code and conversation",
        }
    }
}

/// Represents a package containing prompt templates for a specific agent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    /// Unique identifier for the package
    pub id: String,
    /// Optional URL where the package can be downloaded
    pub url: Option<String>,
    /// Version of the package (semantic versioning)
    pub version: String,
}

impl Package {
    /// Create a new package
    pub fn new<S: Into<String>>(id: S, version: S) -> Self {
        Self {
            id: id.into(),
            url: None,
            version: version.into(),
        }
    }

    /// Create a new package with URL
    pub fn with_url<S: Into<String>>(id: S, url: S, version: S) -> Self {
        Self {
            id: id.into(),
            url: Some(url.into()),
            version: version.into(),
        }
    }

    /// Validate the package structure
    pub fn validate(&self) -> Result<()> {
        // Validate package ID
        if self.id.trim().is_empty() {
            return Err(ConfigError::invalid_package("Package ID cannot be empty"));
        }

        // Package ID should not contain special characters that could cause issues
        if self.id.contains(char::is_whitespace) {
            return Err(ConfigError::invalid_package(format!(
                "Package ID '{}' cannot contain whitespace characters",
                self.id
            )));
        }

        // Package ID should be reasonable length
        if self.id.len() > 100 {
            return Err(ConfigError::invalid_package(format!(
                "Package ID '{}' is too long (max 100 characters)",
                self.id
            )));
        }

        // Validate version format
        if self.version.trim().is_empty() {
            return Err(ConfigError::invalid_package("Package version cannot be empty"));
        }

        // Semantic version validation
        Self::validate_semantic_version(&self.version)?;

        // Validate URL if present
        if let Some(ref url) = self.url {
            Self::validate_url(url)?;
        }

        Ok(())
    }

    /// Validate semantic version format (major.minor.patch with optional pre-release/build)
    fn validate_semantic_version(version: &str) -> Result<()> {
        let trimmed = version.trim();
        
        // Basic format check - should start with digits
        if !trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            return Err(ConfigError::invalid_package(format!(
                "Version '{}' must start with a number (e.g., '1.0.0')",
                version
            )));
        }

        // Check for empty pre-release or build metadata
        if trimmed.contains("-") && trimmed.ends_with("-") {
            return Err(ConfigError::invalid_package(format!(
                "Version '{}' has empty pre-release identifier",
                version
            )));
        }
        
        if trimmed.contains("+") && trimmed.ends_with("+") {
            return Err(ConfigError::invalid_package(format!(
                "Version '{}' has empty build metadata",
                version
            )));
        }

        // Split by build metadata separator first if present
        let (main_part, _build_meta) = trimmed.split_once('+').unwrap_or((trimmed, ""));
        
        // Split by pre-release separator if present
        let (version_part, _pre_release) = main_part.split_once('-').unwrap_or((main_part, ""));
        
        // Split core version into parts
        let parts: Vec<&str> = version_part.split('.').collect();
        
        // Must have at least major version, recommend major.minor.patch
        if parts.is_empty() {
            return Err(ConfigError::invalid_package(format!(
                "Version '{}' is not a valid semantic version (expected format: major.minor.patch)",
                version
            )));
        }

        // For strict semantic versioning, we should have at least major.minor.patch
        if parts.len() < 3 {
            return Err(ConfigError::invalid_package(format!(
                "Version '{}' should have at least major.minor.patch format (e.g., '1.0.0')",
                version
            )));
        }

        // Validate each version component is numeric
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                return Err(ConfigError::invalid_package(format!(
                    "Version '{}' has empty version component at position {}",
                    version, i
                )));
            }
            
            if !part.chars().all(|c| c.is_ascii_digit()) {
                let component = match i {
                    0 => "major",
                    1 => "minor", 
                    2 => "patch",
                    _ => "version component",
                };
                return Err(ConfigError::invalid_package(format!(
                    "Version '{}' has invalid {} component '{}' (must be numeric)",
                    version, component, part
                )));
            }

            // Check for leading zeros (not allowed in semantic versioning)
            if part.len() > 1 && part.starts_with('0') {
                return Err(ConfigError::invalid_package(format!(
                    "Version '{}' component '{}' cannot have leading zeros",
                    version, part
                )));
            }
        }

        Ok(())
    }

    /// Validate URL format if provided
    fn validate_url(url: &str) -> Result<()> {
        let trimmed = url.trim();
        
        if trimmed.is_empty() {
            return Err(ConfigError::invalid_package("Package URL cannot be empty when specified"));
        }

        // Basic URL validation - must start with http:// or https://
        if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            return Err(ConfigError::invalid_package(format!(
                "Package URL '{}' must start with 'http://' or 'https://'",
                url
            )));
        }

        // URL should be reasonable length
        if trimmed.len() > 500 {
            return Err(ConfigError::invalid_package(format!(
                "Package URL is too long (max 500 characters): '{}'",
                url
            )));
        }

        Ok(())
    }
}

/// Main project configuration structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// The selected AI agent for this project
    pub agent: Agent,
    /// List of template packages deployed in this project
    pub packages: Vec<Package>,
    /// Additional project metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ProjectConfig {
    /// Create a new project configuration
    pub fn new(agent: Agent) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert(
            "created_at".to_string(),
            serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
        );

        Self {
            agent,
            packages: Vec::new(),
            metadata,
        }
    }

    /// Create a new project configuration with project name
    pub fn with_project_name<S: Into<String>>(agent: Agent, project_name: S) -> Self {
        let mut config = Self::new(agent);
        config.metadata.insert(
            "project_name".to_string(),
            serde_json::Value::String(project_name.into()),
        );
        config
    }

    /// Add a package to the configuration
    pub fn add_package(&mut self, package: Package) -> Result<()> {
        package.validate()?;

        // Check for duplicate package IDs
        if self.packages.iter().any(|p| p.id == package.id) {
            return Err(ConfigError::invalid_package(format!(
                "Package with ID '{}' already exists",
                package.id
            )));
        }

        self.packages.push(package);
        Ok(())
    }

    /// Get a package by ID
    pub fn get_package(&self, id: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.id == id)
    }

    /// Remove a package by ID
    pub fn remove_package(&mut self, id: &str) -> Option<Package> {
        if let Some(pos) = self.packages.iter().position(|p| p.id == id) {
            Some(self.packages.remove(pos))
        } else {
            None
        }
    }

    /// Validate the entire configuration
    pub fn validate(&self) -> Result<()> {
        // Validate agent (should always be valid due to enum constraints, but check anyway)
        Self::validate_agent(&self.agent)?;

        // Validate all packages
        for (index, package) in self.packages.iter().enumerate() {
            package.validate().map_err(|e| {
                ConfigError::invalid_package(format!("Package at index {}: {}", index, e))
            })?;
        }

        // Check for duplicate package IDs
        Self::validate_unique_package_ids(&self.packages)?;

        // Validate package count limits
        if self.packages.len() > 100 {
            return Err(ConfigError::validation_error(
                "Too many packages (max 100 allowed)"
            ));
        }

        // Validate required metadata fields
        Self::validate_required_metadata(&self.metadata)?;

        // Validate metadata values
        Self::validate_metadata_values(&self.metadata)?;

        Ok(())
    }

    /// Validate agent enum (mostly for completeness)
    fn validate_agent(agent: &Agent) -> Result<()> {
        // Agent enum ensures valid values, but we can add any business logic here
        match agent {
            Agent::Copilot | Agent::Claude => Ok(()),
            // This case should never happen due to enum constraints, but included for completeness
        }
    }

    /// Validate that all package IDs are unique
    fn validate_unique_package_ids(packages: &[Package]) -> Result<()> {
        let mut ids = std::collections::HashSet::new();
        for package in packages {
            if !ids.insert(&package.id) {
                return Err(ConfigError::invalid_package(format!(
                    "Duplicate package ID: '{}'. Each package must have a unique identifier",
                    package.id
                )));
            }
        }
        Ok(())
    }

    /// Validate required metadata fields
    fn validate_required_metadata(metadata: &HashMap<String, serde_json::Value>) -> Result<()> {
        // created_at is required
        if !metadata.contains_key("created_at") {
            return Err(ConfigError::missing_required_field("created_at"));
        }

        // Validate created_at format if present
        if let Some(created_at) = metadata.get("created_at") {
            if let Some(timestamp_str) = created_at.as_str() {
                // Try to parse as RFC3339 timestamp
                if chrono::DateTime::parse_from_rfc3339(timestamp_str).is_err() {
                    return Err(ConfigError::validation_error(format!(
                        "Invalid created_at timestamp format: '{}'. Expected ISO 8601/RFC3339 format",
                        timestamp_str
                    )));
                }
            } else {
                return Err(ConfigError::validation_error(
                    "created_at must be a string in ISO 8601 format"
                ));
            }
        }

        Ok(())
    }

    /// Validate metadata field values
    fn validate_metadata_values(metadata: &HashMap<String, serde_json::Value>) -> Result<()> {
        // Check for reasonable metadata size
        if metadata.len() > 50 {
            return Err(ConfigError::validation_error(
                "Too many metadata fields (max 50 allowed)"
            ));
        }

        for (key, value) in metadata {
            // Validate key format
            if key.trim().is_empty() {
                return Err(ConfigError::validation_error(
                    "Metadata keys cannot be empty"
                ));
            }

            if key.len() > 100 {
                return Err(ConfigError::validation_error(format!(
                    "Metadata key '{}' is too long (max 100 characters)",
                    key
                )));
            }

            // Validate key characters (should be reasonable identifier)
            if key.contains(char::is_control) {
                return Err(ConfigError::validation_error(format!(
                    "Metadata key '{}' contains invalid control characters",
                    key
                )));
            }

            // Validate project_name if present
            if key == "project_name" {
                if let Some(name_str) = value.as_str() {
                    Self::validate_project_name(name_str)?;
                } else {
                    return Err(ConfigError::validation_error(
                        "project_name must be a string"
                    ));
                }
            }

            // Validate value size for strings
            if let Some(str_value) = value.as_str() {
                if str_value.len() > 1000 {
                    return Err(ConfigError::validation_error(format!(
                        "Metadata value for key '{}' is too long (max 1000 characters)",
                        key
                    )));
                }
            }
        }

        Ok(())
    }

    /// Validate project name format
    fn validate_project_name(name: &str) -> Result<()> {
        let trimmed = name.trim();
        
        if trimmed.is_empty() {
            return Err(ConfigError::validation_error(
                "project_name cannot be empty"
            ));
        }

        if trimmed.len() > 200 {
            return Err(ConfigError::validation_error(
                "project_name is too long (max 200 characters)"
            ));
        }

        // Check for control characters
        if trimmed.contains(char::is_control) {
            return Err(ConfigError::validation_error(
                "project_name cannot contain control characters"
            ));
        }

        Ok(())
    }

    /// Validate configuration with detailed error context
    pub fn validate_with_context(&self, context: &str) -> Result<()> {
        self.validate().map_err(|e| {
            ConfigError::validation_error(format!("Configuration validation failed in {}: {}", context, e))
        })
    }

    /// Serialize to JSON string with pretty formatting
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(ConfigError::from)
    }

    /// Deserialize from JSON string
    pub fn from_json_string(json: &str) -> Result<Self> {
        let config: ProjectConfig = serde_json::from_str(json)?;
        config.validate()?;
        Ok(config)
    }

    /// Get the creation timestamp
    pub fn created_at(&self) -> Option<&str> {
        self.metadata
            .get("created_at")?
            .as_str()
    }

    /// Get the project name
    pub fn project_name(&self) -> Option<&str> {
        self.metadata
            .get("project_name")?
            .as_str()
    }

    /// Set project metadata
    pub fn set_metadata<K: Into<String>, V: Into<serde_json::Value>>(&mut self, key: K, value: V) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get project metadata
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_display() {
        assert_eq!(Agent::Copilot.to_string(), "copilot");
        assert_eq!(Agent::Claude.to_string(), "claude");
    }

    #[test]
    fn test_agent_from_str_valid() {
        assert_eq!("copilot".parse::<Agent>().unwrap(), Agent::Copilot);
        assert_eq!("claude".parse::<Agent>().unwrap(), Agent::Claude);
        
        // Test case insensitivity
        assert_eq!("COPILOT".parse::<Agent>().unwrap(), Agent::Copilot);
        assert_eq!("Claude".parse::<Agent>().unwrap(), Agent::Claude);
        assert_eq!("CoPiLoT".parse::<Agent>().unwrap(), Agent::Copilot);
    }

    #[test]
    fn test_agent_from_str_invalid() {
        let result = "invalid".parse::<Agent>();
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("Invalid agent 'invalid'"));
        assert!(error_msg.contains("copilot"));
        assert!(error_msg.contains("claude"));
    }

    #[test]
    fn test_agent_json_serialization() {
        let copilot = Agent::Copilot;
        let claude = Agent::Claude;
        
        let copilot_json = serde_json::to_string(&copilot).unwrap();
        let claude_json = serde_json::to_string(&claude).unwrap();
        
        assert_eq!(copilot_json, "\"copilot\"");
        assert_eq!(claude_json, "\"claude\"");
    }

    #[test]
    fn test_agent_json_deserialization() {
        let copilot: Agent = serde_json::from_str("\"copilot\"").unwrap();
        let claude: Agent = serde_json::from_str("\"claude\"").unwrap();
        
        assert_eq!(copilot, Agent::Copilot);
        assert_eq!(claude, Agent::Claude);
    }

    #[test]
    fn test_agent_json_deserialization_invalid() {
        let result: serde_json::Result<Agent> = serde_json::from_str("\"invalid\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_all() {
        let all_agents = Agent::all();
        assert_eq!(all_agents.len(), 2);
        assert!(all_agents.contains(&Agent::Copilot));
        assert!(all_agents.contains(&Agent::Claude));
    }

    #[test]
    fn test_agent_all_names() {
        let all_names = Agent::all_names();
        assert_eq!(all_names.len(), 2);
        assert!(all_names.contains(&"copilot"));
        assert!(all_names.contains(&"claude"));
    }

    #[test]
    fn test_agent_description() {
        assert!(Agent::Copilot.description().contains("GitHub Copilot"));
        assert!(Agent::Claude.description().contains("Anthropic Claude"));
        assert!(!Agent::Copilot.description().is_empty());
        assert!(!Agent::Claude.description().is_empty());
    }

    #[test]
    fn test_agent_clone_and_equality() {
        let agent1 = Agent::Copilot;
        let agent2 = agent1.clone();
        let agent3 = Agent::Claude;
        
        assert_eq!(agent1, agent2);
        assert_ne!(agent1, agent3);
    }

    #[test]
    fn test_roundtrip_string_conversion() {
        for agent in Agent::all() {
            let string = agent.to_string();
            let parsed: Agent = string.parse().unwrap();
            assert_eq!(agent, parsed);
        }
    }

    #[test]
    fn test_roundtrip_json_conversion() {
        for agent in Agent::all() {
            let json = serde_json::to_string(&agent).unwrap();
            let parsed: Agent = serde_json::from_str(&json).unwrap();
            assert_eq!(agent, parsed);
        }
    }

    // Package tests
    #[test]
    fn test_package_new() {
        let package = Package::new("test-package", "1.0.0");
        assert_eq!(package.id, "test-package");
        assert_eq!(package.version, "1.0.0");
        assert_eq!(package.url, None);
    }

    #[test]
    fn test_package_with_url() {
        let package = Package::with_url("test-package", "https://example.com", "1.0.0");
        assert_eq!(package.id, "test-package");
        assert_eq!(package.version, "1.0.0");
        assert_eq!(package.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_package_validation_valid() {
        let package = Package::new("test-package", "1.0.0");
        assert!(package.validate().is_ok());

        let package_with_url = Package::with_url("test", "https://example.com", "2.1.3");
        assert!(package_with_url.validate().is_ok());
    }

    #[test]
    fn test_package_validation_empty_id() {
        let package = Package::new("", "1.0.0");
        let result = package.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Package ID cannot be empty"));
    }

    #[test]
    fn test_package_validation_empty_version() {
        let package = Package::new("test", "");
        let result = package.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Package version cannot be empty"));
    }

    #[test]
    fn test_package_validation_invalid_version() {
        let package = Package::new("test", "invalid-version");
        let result = package.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must start with a number"));
    }

    #[test]
    fn test_package_json_serialization() {
        let package = Package::with_url("test-package", "https://example.com", "1.0.0");
        let json = serde_json::to_string_pretty(&package).unwrap();
        
        assert!(json.contains("\"id\": \"test-package\""));
        assert!(json.contains("\"url\": \"https://example.com\""));
        assert!(json.contains("\"version\": \"1.0.0\""));
    }

    #[test]
    fn test_package_json_deserialization() {
        let json = r#"{
            "id": "test-package",
            "url": "https://example.com",
            "version": "1.0.0"
        }"#;
        
        let package: Package = serde_json::from_str(json).unwrap();
        assert_eq!(package.id, "test-package");
        assert_eq!(package.url, Some("https://example.com".to_string()));
        assert_eq!(package.version, "1.0.0");
    }

    // ProjectConfig tests
    #[test]
    fn test_project_config_new() {
        let config = ProjectConfig::new(Agent::Copilot);
        assert_eq!(config.agent, Agent::Copilot);
        assert!(config.packages.is_empty());
        assert!(config.metadata.contains_key("created_at"));
    }

    #[test]
    fn test_project_config_with_project_name() {
        let config = ProjectConfig::with_project_name(Agent::Claude, "my-project");
        assert_eq!(config.agent, Agent::Claude);
        assert_eq!(config.project_name(), Some("my-project"));
        assert!(config.created_at().is_some());
    }

    #[test]
    fn test_project_config_add_package() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        let package = Package::new("test-package", "1.0.0");
        
        assert!(config.add_package(package).is_ok());
        assert_eq!(config.packages.len(), 1);
        assert_eq!(config.packages[0].id, "test-package");
    }

    #[test]
    fn test_project_config_add_duplicate_package() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        let package1 = Package::new("test-package", "1.0.0");
        let package2 = Package::new("test-package", "2.0.0");
        
        assert!(config.add_package(package1).is_ok());
        let result = config.add_package(package2);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_project_config_get_package() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        let package = Package::new("test-package", "1.0.0");
        config.add_package(package).unwrap();
        
        let found = config.get_package("test-package");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "test-package");
        
        let not_found = config.get_package("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_project_config_remove_package() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        let package = Package::new("test-package", "1.0.0");
        config.add_package(package).unwrap();
        
        let removed = config.remove_package("test-package");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, "test-package");
        assert!(config.packages.is_empty());
        
        let not_removed = config.remove_package("nonexistent");
        assert!(not_removed.is_none());
    }

    #[test]
    fn test_project_config_validation() {
        let config = ProjectConfig::new(Agent::Copilot);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_project_config_json_serialization() {
        let mut config = ProjectConfig::with_project_name(Agent::Copilot, "test-project");
        let package = Package::with_url("test-package", "https://example.com", "1.0.0");
        config.add_package(package).unwrap();
        
        let json = config.to_json_string().unwrap();
        
        assert!(json.contains("\"agent\": \"copilot\""));
        assert!(json.contains("\"test-package\""));
        assert!(json.contains("\"project_name\": \"test-project\""));
        assert!(json.contains("\"created_at\""));
    }

    #[test]
    fn test_project_config_json_deserialization() {
        let json = r#"{
            "agent": "claude",
            "packages": [
                {
                    "id": "test-package",
                    "url": "https://example.com",
                    "version": "1.0.0"
                }
            ],
            "metadata": {
                "created_at": "2025-09-12T00:00:00Z",
                "project_name": "test-project"
            }
        }"#;
        
        let config = ProjectConfig::from_json_string(json).unwrap();
        assert_eq!(config.agent, Agent::Claude);
        assert_eq!(config.packages.len(), 1);
        assert_eq!(config.packages[0].id, "test-package");
        assert_eq!(config.project_name(), Some("test-project"));
        assert_eq!(config.created_at(), Some("2025-09-12T00:00:00Z"));
    }

    #[test]
    fn test_project_config_roundtrip_json() {
        let mut original = ProjectConfig::with_project_name(Agent::Claude, "test-project");
        let package = Package::with_url("test-package", "https://example.com", "1.0.0");
        original.add_package(package).unwrap();
        original.set_metadata("custom_field", "custom_value");
        
        let json = original.to_json_string().unwrap();
        let deserialized = ProjectConfig::from_json_string(&json).unwrap();
        
        assert_eq!(original.agent, deserialized.agent);
        assert_eq!(original.packages, deserialized.packages);
        assert_eq!(original.project_name(), deserialized.project_name());
        assert_eq!(original.get_metadata("custom_field"), deserialized.get_metadata("custom_field"));
    }

    #[test]
    fn test_project_config_metadata_operations() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        
        config.set_metadata("test_key", "test_value");
        config.set_metadata("number_key", 42);
        
        assert_eq!(config.get_metadata("test_key"), Some(&serde_json::Value::String("test_value".to_string())));
        assert_eq!(config.get_metadata("number_key"), Some(&serde_json::Value::Number(serde_json::Number::from(42))));
        assert_eq!(config.get_metadata("nonexistent"), None);
    }

    // Enhanced validation tests
    #[test]
    fn test_package_validation_whitespace_in_id() {
        let package = Package::new("test package", "1.0.0");
        let result = package.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot contain whitespace"));
    }

    #[test]
    fn test_package_validation_long_id() {
        let long_id = "a".repeat(101);
        let package = Package::new(long_id, "1.0.0".to_string());
        let result = package.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_package_semantic_version_validation() {
        // Valid versions
        let valid_versions = vec![
            "1.0.0", "0.1.0", "10.20.30", "1.2.3-alpha", "1.0.0+build", "1.2.3-beta.1+build.2"
        ];
        for version in valid_versions {
            let package = Package::new("test", version);
            assert!(package.validate().is_ok(), "Version '{}' should be valid", version);
        }

        // Invalid versions
        let invalid_versions = vec![
            "1.0", "1", "v1.0.0", "01.0.0", "1.01.0", "1.0.01",
            "1.0.-1", "1..0", ".1.0.0", "1.0.0.", ""
        ];
        for version in invalid_versions {
            let package = Package::new("test", version);
            let result = package.validate();
            assert!(result.is_err(), "Version '{}' should be invalid", version);
        }
    }

    #[test]
    fn test_package_url_validation() {
        // Valid URLs
        let valid_urls = vec![
            "https://github.com/user/repo",
            "http://example.com",
            "https://api.example.com/v1/packages"
        ];
        for url in valid_urls {
            let package = Package::with_url("test", url, "1.0.0");
            assert!(package.validate().is_ok(), "URL '{}' should be valid", url);
        }

        // Invalid URLs
        let long_url = "https://".repeat(200);
        let invalid_urls = vec![
            "ftp://example.com",
            "github.com/user/repo", 
            "not-a-url",
            "",
            &long_url // Too long
        ];
        for url in invalid_urls {
            let package = Package::with_url("test", url, "1.0.0");
            let result = package.validate();
            assert!(result.is_err(), "URL '{}' should be invalid", url);
        }
    }

    #[test]
    fn test_project_config_package_count_limit() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        
        // Add maximum allowed packages
        for i in 0..100 {
            let package = Package::new(format!("package-{}", i), "1.0.0".to_string());
            config.add_package(package).unwrap();
        }
        
        assert!(config.validate().is_ok());
        
        // Try to add one more
        let extra_package = Package::new("package-extra", "1.0.0");
        config.packages.push(extra_package); // Bypass add_package validation
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Too many packages"));
    }

    #[test]
    fn test_project_config_invalid_timestamp() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        config.set_metadata("created_at", "invalid-timestamp");
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid created_at timestamp"));
    }

    #[test]
    fn test_project_config_metadata_limits() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        
        // Add maximum allowed metadata fields (49 + 1 created_at = 50)
        for i in 0..49 {
            config.set_metadata(&format!("key{}", i), "value");
        }
        
        assert!(config.validate().is_ok());
        
        // Add one more to exceed limit
        config.set_metadata("extra_key", "value");
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Too many metadata fields"));
    }

    #[test]
    fn test_project_config_empty_metadata_key() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        config.metadata.insert("".to_string(), serde_json::Value::String("test".to_string()));
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Metadata keys cannot be empty"));
    }

    #[test]
    fn test_project_config_long_metadata_key() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        let long_key = "a".repeat(101);
        config.metadata.insert(long_key, serde_json::Value::String("test".to_string()));
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_project_config_invalid_project_name() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        
        // Empty project name
        config.set_metadata("project_name", "");
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("project_name cannot be empty"));
        
        // Too long project name
        let long_name = "a".repeat(201);
        config.set_metadata("project_name", long_name);
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("project_name is too long"));
    }

    #[test]
    fn test_project_config_validation_with_context() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        config.set_metadata("created_at", "invalid");
        
        let result = config.validate_with_context("test context");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test context"));
    }

    #[test]
    fn test_package_validation_error_with_index() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        
        // Add valid package first
        config.add_package(Package::new("valid", "1.0.0")).unwrap();
        
        // Add invalid package directly to bypass add_package validation
        config.packages.push(Package::new("", "1.0.0")); // Invalid: empty ID
        
        let result = config.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Package at index 1"));
        assert!(error_msg.contains("Package ID cannot be empty"));
    }

    #[test]
    fn test_duplicate_package_ids_detailed_error() {
        let mut config = ProjectConfig::new(Agent::Copilot);
        
        // Add packages with same ID directly to bypass add_package validation
        config.packages.push(Package::new("duplicate-id", "1.0.0"));
        config.packages.push(Package::new("duplicate-id", "2.0.0"));
        
        let result = config.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate package ID: 'duplicate-id'"));
        assert!(error_msg.contains("Each package must have a unique identifier"));
    }

    #[test]
    fn test_semantic_version_validation_edge_cases() {
        let test_cases = vec![
            ("1.2.3-alpha.1", true),
            ("1.2.3+20130313144700", true),
            ("1.2.3-beta+exp.sha.5114f85", true),
            ("10.2.3", true),
            ("1.2.3-0123", true), // Leading zeros in pre-release are allowed
            ("1.2.3-0123.0123", true),
            ("1.2.3-", false), // Empty pre-release
            ("1.2.3+", false), // Empty build metadata  
            ("1.2.3.4", true),  // More than 3 components allowed
            ("1", false),       // Major only - now invalid
            ("1.2", false),     // Major.minor only - now invalid
        ];

        for (version, should_be_valid) in test_cases {
            let package = Package::new("test", version);
            let result = package.validate();
            if should_be_valid {
                assert!(result.is_ok(), "Version '{}' should be valid but got error: {:?}", 
                       version, result.err());
            } else {
                assert!(result.is_err(), "Version '{}' should be invalid but was accepted", version);
            }
        }
    }
}