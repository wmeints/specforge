use clap::Args;
use std::path::PathBuf;
use dialoguer::{Select, theme::ColorfulTheme};
use crate::config::{Agent, ProjectConfig, Package};
use crate::file_ops::FileOps;
use crate::error::{ConfigError, Result};

/// Initialize a new Reforge project with agent configuration
#[derive(Args)]
pub struct InitCommand {
    /// The AI agent to configure for this project
    #[arg(short, long, value_enum)]
    pub agent: Option<AgentType>,
    
    /// Output directory for the configuration file
    #[arg(short, long, default_value = ".", value_parser = validate_output_directory)]
    pub output_directory: PathBuf,
    
    /// Project name (optional)
    #[arg(short, long)]
    pub project_name: Option<String>,
    
    /// Force overwrite existing configuration
    #[arg(short, long)]
    pub force: bool,
}

/// Supported AI agent types for CLI
#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum AgentType {
    /// GitHub Copilot
    Copilot,
    /// Anthropic Claude
    Claude,
}

impl From<AgentType> for Agent {
    fn from(agent_type: AgentType) -> Self {
        match agent_type {
            AgentType::Copilot => Agent::Copilot,
            AgentType::Claude => Agent::Claude,
        }
    }
}

impl From<Agent> for AgentType {
    fn from(agent: Agent) -> Self {
        match agent {
            Agent::Copilot => AgentType::Copilot,
            Agent::Claude => AgentType::Claude,
        }
    }
}

/// Validate output directory path with comprehensive checks
fn validate_output_directory(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    
    // Validate and canonicalize the path
    let canonical_path = FileOps::validate_and_canonicalize_path(&path)?;
    
    // If the path exists, verify it's a directory
    if canonical_path.exists() {
        if !canonical_path.is_dir() {
            return Err(ConfigError::validation_error(format!(
                "Output path '{}' exists but is not a directory", 
                canonical_path.display()
            )));
        }
        
        // Check write permissions for existing directory
        FileOps::check_write_permissions(&canonical_path).map_err(|e| {
            ConfigError::validation_error(format!(
                "Output directory '{}' is not writable: {}", 
                canonical_path.display(), e
            ))
        })?;
    } else {
        // For non-existent paths, check if parent directories exist and are writable
        if let Some(parent) = canonical_path.parent() {
            if parent.exists() {
                if !parent.is_dir() {
                    return Err(ConfigError::validation_error(format!(
                        "Parent path '{}' exists but is not a directory",
                        parent.display()
                    )));
                }
                
                // Check write permissions on parent directory
                FileOps::check_write_permissions(parent).map_err(|e| {
                    ConfigError::validation_error(format!(
                        "Cannot create directory in '{}': {}", 
                        parent.display(), e
                    ))
                })?;
            }
            // If parent doesn't exist, that's okay - we'll create the full path later
        }
    }
    
    Ok(canonical_path)
}

impl InitCommand {
    /// Execute the init command
    pub fn execute(&self) -> Result<()> {
        println!("ℹ️  Initializing Reforge project...");

        // Validate command arguments with context
        self.validate()
            .map_err(|e| e.add_context("command validation", "Checking init command parameters"))?;

        // Determine agent (either from flag or interactive selection)
        let agent = self.determine_agent()
            .map_err(|e| e.add_context("agent selection", "Determining which AI agent to configure"))?;
        println!("ℹ️  Selected agent: {}", agent);

        // Create project configuration with enhanced error context
        let config = self.create_project_config(agent.clone())
            .map_err(|e| e.add_context("configuration creation",
                &format!("Creating configuration for {} agent", agent)))?;

        // Ensure output directory exists, with enhanced error handling
        if !self.output_directory.exists() {
            println!("ℹ️  Creating output directory: {}", self.output_directory.display());
            FileOps::ensure_directory_exists(&self.output_directory)
                .map_err(|e| e.add_context("directory creation",
                    &format!("Creating output directory at {}", self.output_directory.display())))?;
        }

        // Write configuration file with context-aware error handling
        let config_path = FileOps::write_config_to_directory_with_confirmation(
            &config,
            &self.output_directory,
            self.force
        ).map_err(|e| e.add_context("configuration file writing",
            &format!("Writing .reforge.json to {}", self.output_directory.display())))?;

        // Display success message
        println!("✅ Successfully created Reforge configuration at: {}", config_path.display());

        // Display next steps
        self.display_next_steps(&agent);

        Ok(())
    }
    
    /// Validate command arguments
    fn validate(&self) -> Result<()> {
        // Validate project name if provided
        if let Some(ref name) = self.project_name {
            if name.trim().is_empty() {
                return Err(ConfigError::validation_error(
                    "Project name cannot be empty"
                ));
            }
            
            if name.len() > 200 {
                return Err(ConfigError::validation_error(
                    "Project name is too long (max 200 characters)"
                ));
            }
        }
        
        // Output directory validation is handled by clap value_parser
        
        Ok(())
    }
    
    /// Determine which agent to use (from flag or interactive prompt)
    fn determine_agent(&self) -> Result<Agent> {
        if let Some(agent_type) = &self.agent {
            // Agent specified via flag
            Ok(Agent::from(agent_type.clone()))
        } else {
            // Interactive agent selection
            self.interactive_agent_selection()
        }
    }
    
    /// Perform interactive agent selection using dialoguer
    fn interactive_agent_selection(&self) -> Result<Agent> {
        println!("ℹ️  No agent specified. Please select an AI agent for this project:");
        println!();
        
        let agents = Agent::all();
        let agent_options: Vec<String> = agents
            .iter()
            .map(|agent| format!("{} - {}", agent, agent.description()))
            .collect();
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your AI agent")
            .default(0)
            .items(&agent_options)
            .interact_opt()
            .map_err(|e| ConfigError::io_error(format!("Failed to read user input: {}", e)))?;
        
        match selection {
            Some(index) => {
                let selected_agent = agents[index].clone();
                println!();
                println!("✅ Selected agent: {}", selected_agent);
                Ok(selected_agent)
            }
            None => {
                // User cancelled (Ctrl+C or Esc)
                println!();
                println!("❌ Agent selection cancelled by user");
                Err(ConfigError::user_cancelled("Agent selection was cancelled"))
            }
        }
    }
    
    /// Create project configuration based on command arguments
    fn create_project_config(&self, agent: Agent) -> Result<ProjectConfig> {
        let mut config = if let Some(ref project_name) = self.project_name {
            ProjectConfig::with_project_name(agent, project_name)
        } else {
            ProjectConfig::new(agent)
        };
        
        // Add default template packages based on agent
        let default_package = self.create_default_package(&config.agent);
        config.add_package(default_package)?;
        
        // Set additional metadata
        config.set_metadata("initialized_by", "reforge-cli");
        config.set_metadata("version", env!("CARGO_PKG_VERSION"));
        
        // Validate the configuration
        config.validate()?;
        
        Ok(config)
    }
    
    /// Create default template package based on selected agent
    ///
    /// Creates a package entry with:
    /// - Meaningful package ID specific to the agent
    /// - Current crate version for version tracking
    /// - Proper structure for future template deployment features
    fn create_default_package(&self, agent: &Agent) -> Package {
        let package_version = env!("CARGO_PKG_VERSION");

        match agent {
            Agent::Copilot => Package::new(
                "reforge-copilot-templates",
                package_version
            ),
            Agent::Claude => Package::new(
                "reforge-claude-templates",
                package_version
            ),
        }
    }

    /// Create multiple template packages for an agent (if needed in the future)
    ///
    /// This method allows for creating multiple packages per agent, supporting
    /// different template categories or specialized packages.
    #[allow(dead_code)] // Future feature
    fn create_agent_packages(&self, agent: &Agent) -> Vec<Package> {
        let package_version = env!("CARGO_PKG_VERSION");

        match agent {
            Agent::Copilot => vec![
                Package::new("reforge-copilot-templates", package_version),
                // Future: Additional packages like "reforge-copilot-advanced-templates"
            ],
            Agent::Claude => vec![
                Package::new("reforge-claude-templates", package_version),
                // Future: Additional packages like "reforge-claude-advanced-templates"
            ],
        }
    }
    
    /// Display helpful next steps to the user
    fn display_next_steps(&self, agent: &Agent) {
        println!();
        println!("🎉 Next steps:");
        println!("   1. Review the generated .reforge.json configuration");
        println!("   2. Customize the configuration as needed");
        println!("   3. Start using your AI agent with the configured templates");
        
        match agent {
            Agent::Copilot => {
                println!("   4. Make sure GitHub Copilot is enabled in your editor");
            }
            Agent::Claude => {
                println!("   4. Make sure Claude Code extension is installed and configured");
            }
        }
    }
    
    /// Get a summary of the command configuration for display
    pub fn get_summary(&self) -> String {
        let mut summary = Vec::new();
        
        if let Some(ref agent) = self.agent {
            summary.push(format!("Agent: {:?}", agent));
        } else {
            summary.push("Agent: Interactive selection".to_string());
        }
        
        summary.push(format!("Output directory: {}", self.output_directory.display()));
        
        if let Some(ref name) = self.project_name {
            summary.push(format!("Project name: {}", name));
        }
        
        if self.force {
            summary.push("Force overwrite: enabled".to_string());
        }
        
        summary.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_agent_type_conversion() {
        // Test AgentType to Agent conversion
        assert_eq!(Agent::from(AgentType::Copilot), Agent::Copilot);
        assert_eq!(Agent::from(AgentType::Claude), Agent::Claude);
        
        // Test Agent to AgentType conversion
        assert_eq!(AgentType::from(Agent::Copilot), AgentType::Copilot);
        assert_eq!(AgentType::from(Agent::Claude), AgentType::Claude);
    }
    
    #[test]
    fn test_validate_output_directory() {
        use tempfile::TempDir;
        
        // Valid paths
        assert!(validate_output_directory(".").is_ok());
        assert!(validate_output_directory("/tmp").is_ok());
        
        // Test with temporary directory
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_string_lossy();
        assert!(validate_output_directory(&temp_path).is_ok());
        
        // Test with nested path under temp directory
        let nested_path = temp_dir.path().join("nested").join("path");
        let nested_str = nested_path.to_string_lossy();
        assert!(validate_output_directory(&nested_str).is_ok());
        
        // The validator should handle path canonicalization
        let result = validate_output_directory("../test");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_output_directory_file_conflict() {
        use tempfile::TempDir;
        use std::fs;
        
        let temp_dir = TempDir::new().unwrap();
        
        // Create a regular file
        let file_path = temp_dir.path().join("not_a_directory");
        fs::write(&file_path, "test content").unwrap();
        
        // Try to use the file path as a directory - should fail
        let file_str = file_path.to_string_lossy();
        let result = validate_output_directory(&file_str);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exists but is not a directory"));
    }
    
    #[test]
    fn test_validate_output_directory_parent_file_conflict() {
        use tempfile::TempDir;
        use std::fs;
        
        let temp_dir = TempDir::new().unwrap();
        
        // Create a regular file
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "test content").unwrap();
        
        // Try to create a directory under the file - should fail
        let invalid_dir = file_path.join("subdir");
        let invalid_str = invalid_dir.to_string_lossy();
        let result = validate_output_directory(&invalid_str);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exists but is not a directory"));
    }
    
    #[test]
    fn test_init_command_validation() {
        // Valid command
        let cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: PathBuf::from("."),
            project_name: Some("test-project".to_string()),
            force: false,
        };
        assert!(cmd.validate().is_ok());
        
        // Empty project name should fail
        let cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: PathBuf::from("."),
            project_name: Some("".to_string()),
            force: false,
        };
        assert!(cmd.validate().is_err());
        
        // Too long project name should fail
        let cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: PathBuf::from("."),
            project_name: Some("a".repeat(201)),
            force: false,
        };
        assert!(cmd.validate().is_err());
    }
    
    #[test]
    fn test_determine_agent() {
        // Agent specified via flag
        let cmd = InitCommand {
            agent: Some(AgentType::Claude),
            output_directory: PathBuf::from("."),
            project_name: None,
            force: false,
        };
        assert_eq!(cmd.determine_agent().unwrap(), Agent::Claude);
        
        // No agent specified requires interactive selection which we can't test in unit tests
        // Interactive selection tests would be in integration tests
    }
    
    #[test]
    fn test_create_project_config() {
        let cmd = InitCommand {
            agent: Some(AgentType::Claude),
            output_directory: PathBuf::from("."),
            project_name: Some("test-project".to_string()),
            force: false,
        };
        
        let config = cmd.create_project_config(Agent::Claude).unwrap();
        
        assert_eq!(config.agent, Agent::Claude);
        assert_eq!(config.project_name(), Some("test-project"));
        assert_eq!(config.packages.len(), 1);
        assert_eq!(config.packages[0].id, "reforge-claude-templates");
        assert!(config.get_metadata("initialized_by").is_some());
        assert!(config.get_metadata("version").is_some());
    }
    
    #[test]
    fn test_create_default_package() {
        let cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: PathBuf::from("."),
            project_name: None,
            force: false,
        };

        let expected_version = env!("CARGO_PKG_VERSION");

        let copilot_package = cmd.create_default_package(&Agent::Copilot);
        assert_eq!(copilot_package.id, "reforge-copilot-templates");
        assert_eq!(copilot_package.version, expected_version);

        let claude_package = cmd.create_default_package(&Agent::Claude);
        assert_eq!(claude_package.id, "reforge-claude-templates");
        assert_eq!(claude_package.version, expected_version);
    }
    
    #[test]
    fn test_get_summary() {
        let cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: PathBuf::from("/test/dir"),
            project_name: Some("my-project".to_string()),
            force: true,
        };
        
        let summary = cmd.get_summary();
        assert!(summary.contains("Agent: Copilot"));
        assert!(summary.contains("Output directory: /test/dir"));
        assert!(summary.contains("Project name: my-project"));
        assert!(summary.contains("Force overwrite: enabled"));
    }
    
    #[test]
    fn test_init_command_execution_dry_run() {
        let temp_dir = TempDir::new().unwrap();
        
        let cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: temp_dir.path().to_path_buf(),
            project_name: Some("test-project".to_string()),
            force: false,
        };
        
        // This should work in the temporary directory
        let result = cmd.execute();
        assert!(result.is_ok());
        
        // Verify config file was created
        assert!(FileOps::config_exists_in_directory(temp_dir.path()));
    }
    
    #[test]
    fn test_force_overwrite_behavior() {
        let temp_dir = TempDir::new().unwrap();

        // Create initial config
        let cmd1 = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: temp_dir.path().to_path_buf(),
            project_name: None,
            force: false,
        };
        cmd1.execute().unwrap();

        // Try to create again without force - would normally prompt user
        // In test environment, we can't test interactive confirmation easily,
        // so we skip this part of the test

        // Try to create again with force - should succeed
        let cmd3 = InitCommand {
            agent: Some(AgentType::Claude),
            output_directory: temp_dir.path().to_path_buf(),
            project_name: None,
            force: true,
        };
        assert!(cmd3.execute().is_ok());

        // Verify the config was overwritten (agent should be Claude now)
        let config = FileOps::read_config_from_directory(temp_dir.path()).unwrap();
        assert_eq!(config.agent, Agent::Claude);
    }

    #[test]
    fn test_packages_array_creation_comprehensive() {
        let temp_dir = TempDir::new().unwrap();

        // Test Copilot packages array creation
        let copilot_cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: temp_dir.path().join("copilot").to_path_buf(),
            project_name: Some("copilot-project".to_string()),
            force: false,
        };
        copilot_cmd.execute().unwrap();

        let copilot_config = FileOps::read_config_from_directory(&temp_dir.path().join("copilot")).unwrap();

        // Verify packages array structure
        assert_eq!(copilot_config.packages.len(), 1);
        let copilot_package = &copilot_config.packages[0];

        // Test acceptance criteria:
        // - Packages array is created with appropriate template package entries
        assert_eq!(copilot_package.id, "reforge-copilot-templates");

        // - Package IDs are meaningful and consistent
        assert!(copilot_package.id.contains("copilot"));
        assert!(copilot_package.id.contains("templates"));

        // - Version information is accurate and follows semantic versioning
        assert_eq!(copilot_package.version, env!("CARGO_PKG_VERSION"));
        assert!(copilot_package.validate().is_ok());

        // - Package structure supports future template deployment features
        assert!(copilot_package.url.is_none()); // Ready for future URL assignment

        // Test Claude packages array creation
        let claude_cmd = InitCommand {
            agent: Some(AgentType::Claude),
            output_directory: temp_dir.path().join("claude").to_path_buf(),
            project_name: Some("claude-project".to_string()),
            force: false,
        };
        claude_cmd.execute().unwrap();

        let claude_config = FileOps::read_config_from_directory(&temp_dir.path().join("claude")).unwrap();

        // Verify Claude packages array
        assert_eq!(claude_config.packages.len(), 1);
        let claude_package = &claude_config.packages[0];

        // - Different agents can have different default packages if needed
        assert_eq!(claude_package.id, "reforge-claude-templates");
        assert_ne!(claude_package.id, copilot_package.id);

        // - Version information is consistent across agents
        assert_eq!(claude_package.version, copilot_package.version);

        // Test JSON schema compliance
        let json_string = copilot_config.to_json_string().unwrap();
        let _parsed: ProjectConfig = serde_json::from_str(&json_string).unwrap();

        // Verify JSON contains expected structure
        assert!(json_string.contains("\"packages\""));
        assert!(json_string.contains("\"id\""));
        assert!(json_string.contains("\"version\""));
        assert!(json_string.contains("reforge-copilot-templates"));
    }

    #[test]
    fn test_package_versioning_accuracy() {
        let cmd = InitCommand {
            agent: Some(AgentType::Copilot),
            output_directory: PathBuf::from("."),
            project_name: None,
            force: false,
        };

        let package = cmd.create_default_package(&Agent::Copilot);

        // Version should match current crate version exactly
        assert_eq!(package.version, env!("CARGO_PKG_VERSION"));

        // Version should follow semantic versioning
        let version_parts: Vec<&str> = package.version.split('.').collect();
        assert!(version_parts.len() >= 3, "Version should have at least major.minor.patch");

        // Each version component should be numeric
        for part in &version_parts[0..3] {
            assert!(part.parse::<u32>().is_ok(), "Version component '{}' should be numeric", part);
        }

        // Package should pass validation
        assert!(package.validate().is_ok());
    }
}