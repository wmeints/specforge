use clap::Args;
use std::path::PathBuf;
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

/// Validate output directory path
fn validate_output_directory(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    
    // Validate and canonicalize the path
    FileOps::validate_and_canonicalize_path(&path)
}

impl InitCommand {
    /// Execute the init command
    pub fn execute(&self) -> Result<()> {
        println!("ℹ️  Initializing Reforge project...");
        
        // Validate command arguments
        self.validate()?;
        
        // Check if config already exists (unless force is specified)
        if !self.force && FileOps::config_exists_in_directory(&self.output_directory) {
            let config_path = FileOps::get_config_path(&self.output_directory);
            return Err(ConfigError::file_exists(config_path));
        }
        
        // Determine agent (either from flag or interactive selection)
        let agent = self.determine_agent()?;
        println!("ℹ️  Selected agent: {}", agent);
        
        // Create project configuration
        let config = self.create_project_config(agent)?;
        
        // Ensure output directory exists
        FileOps::ensure_directory_exists(&self.output_directory)?;
        
        // Write configuration file
        let config_path = FileOps::write_config_to_directory(&config, &self.output_directory)?;
        
        // Display success message
        println!("✅ Successfully created Reforge configuration at: {}", config_path.display());
        
        // Display next steps
        self.display_next_steps();
        
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
            // Interactive agent selection (placeholder for now)
            println!("ℹ️  No agent specified - interactive selection will be implemented");
            
            // For now, default to Copilot if no agent specified
            // This will be replaced with interactive selection in the next task
            Ok(Agent::Copilot)
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
    fn create_default_package(&self, agent: &Agent) -> Package {
        match agent {
            Agent::Copilot => Package::new(
                "reforge-copilot-templates",
                "1.0.0"
            ),
            Agent::Claude => Package::new(
                "reforge-claude-templates", 
                "1.0.0"
            ),
        }
    }
    
    /// Display helpful next steps to the user
    fn display_next_steps(&self) {
        println!();
        println!("🎉 Next steps:");
        println!("   1. Review the generated .reforge.json configuration");
        println!("   2. Customize the configuration as needed");
        println!("   3. Start using your AI agent with the configured templates");
        
        match self.determine_agent() {
            Ok(Agent::Copilot) => {
                println!("   4. Make sure GitHub Copilot is enabled in your editor");
            }
            Ok(Agent::Claude) => {
                println!("   4. Make sure Claude Code extension is installed and configured");
            }
            Err(_) => {}
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
        // Valid paths
        assert!(validate_output_directory(".").is_ok());
        assert!(validate_output_directory("./test").is_ok());
        assert!(validate_output_directory("/tmp").is_ok());
        
        // The validator should handle path canonicalization
        let result = validate_output_directory("../test");
        assert!(result.is_ok());
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
        
        // No agent specified (defaults to Copilot for now)
        let cmd = InitCommand {
            agent: None,
            output_directory: PathBuf::from("."),
            project_name: None,
            force: false,
        };
        assert_eq!(cmd.determine_agent().unwrap(), Agent::Copilot);
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
        
        let copilot_package = cmd.create_default_package(&Agent::Copilot);
        assert_eq!(copilot_package.id, "reforge-copilot-templates");
        assert_eq!(copilot_package.version, "1.0.0");
        
        let claude_package = cmd.create_default_package(&Agent::Claude);
        assert_eq!(claude_package.id, "reforge-claude-templates");
        assert_eq!(claude_package.version, "1.0.0");
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
        
        // Try to create again without force - should fail
        let cmd2 = InitCommand {
            agent: Some(AgentType::Claude),
            output_directory: temp_dir.path().to_path_buf(),
            project_name: None,
            force: false,
        };
        assert!(cmd2.execute().is_err());
        
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
}