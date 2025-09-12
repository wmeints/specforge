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
        if self.id.trim().is_empty() {
            return Err(ConfigError::invalid_package("Package ID cannot be empty"));
        }

        if self.version.trim().is_empty() {
            return Err(ConfigError::invalid_package("Package version cannot be empty"));
        }

        // Basic semantic version validation (simplified)
        if !self.version.chars().any(|c| c.is_ascii_digit()) {
            return Err(ConfigError::invalid_package(format!(
                "Invalid version format: '{}'. Expected semantic version like '1.0.0'",
                self.version
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
        // Validate all packages
        for package in &self.packages {
            package.validate()?;
        }

        // Check for duplicate package IDs
        let mut ids = std::collections::HashSet::new();
        for package in &self.packages {
            if !ids.insert(&package.id) {
                return Err(ConfigError::invalid_package(format!(
                    "Duplicate package ID: '{}'",
                    package.id
                )));
            }
        }

        // Validate required metadata
        if !self.metadata.contains_key("created_at") {
            return Err(ConfigError::missing_required_field("created_at"));
        }

        Ok(())
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
        assert!(result.unwrap_err().to_string().contains("Invalid version format"));
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
}