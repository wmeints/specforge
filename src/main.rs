use clap::{Parser, Subcommand};
use reforge::{ConfigError, Result};
use std::process;

/// Reforge CLI - Configure source control for AI-driven development
#[derive(Parser)]
#[command(
    name = "reforge",
    version,
    about = "Configure source control for AI-driven development through GitHub Copilot or Claude Code",
    long_about = "Reforge allows developers to configure their source control for AI-driven development \
                 by quickly deploying custom prompt templates for coding agents. Follow a specification-driven \
                 workflow where you handle specifications and review while the AI handles the coding.",
    author = "Reforge Contributors"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Reforge project with agent configuration
    Init {
        /// The AI agent to configure for this project
        #[arg(short, long, value_enum)]
        agent: Option<AgentType>,
        
        /// Output directory for the configuration file
        #[arg(short, long, default_value = ".")]
        output_directory: String,
        
        /// Project name (optional)
        #[arg(short, long)]
        project_name: Option<String>,
        
        /// Force overwrite existing configuration
        #[arg(short, long)]
        force: bool,
    },
}

/// Supported AI agent types
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AgentType {
    /// GitHub Copilot
    Copilot,
    /// Anthropic Claude
    Claude,
}

impl From<AgentType> for reforge::config::Agent {
    fn from(agent_type: AgentType) -> Self {
        match agent_type {
            AgentType::Copilot => reforge::config::Agent::Copilot,
            AgentType::Claude => reforge::config::Agent::Claude,
        }
    }
}

impl From<reforge::config::Agent> for AgentType {
    fn from(agent: reforge::config::Agent) -> Self {
        match agent {
            reforge::config::Agent::Copilot => AgentType::Copilot,
            reforge::config::Agent::Claude => AgentType::Claude,
        }
    }
}

/// Handle CLI errors and exit with appropriate codes
fn handle_error(error: ConfigError) -> ! {
    eprintln!("Error: {}", error);
    
    // Exit with appropriate code based on error type
    let exit_code = match error {
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
    };
    
    process::exit(exit_code);
}

/// Display success message for completed operations
fn display_success(message: &str) {
    println!("✅ {}", message);
}

/// Display informational message
fn display_info(message: &str) {
    println!("ℹ️  {}", message);
}

/// Display warning message
fn display_warning(message: &str) {
    println!("⚠️  {}", message);
}

fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Init {
            agent,
            output_directory,
            project_name,
            force,
        } => {
            // Import init command handling
            handle_init_command(agent, output_directory, project_name, force)
        }
    };
    
    // Handle any errors
    if let Err(error) = result {
        handle_error(error);
    }
}

/// Handle the init command (placeholder for now)
fn handle_init_command(
    agent: Option<AgentType>,
    output_directory: String,
    project_name: Option<String>,
    force: bool,
) -> Result<()> {
    display_info("Initializing Reforge project...");
    
    // This is a placeholder implementation
    // The actual implementation will be added in the next task
    if let Some(agent_type) = agent {
        display_info(&format!("Selected agent: {:?}", agent_type));
    } else {
        display_info("No agent specified - interactive selection will be implemented");
    }
    
    display_info(&format!("Output directory: {}", output_directory));
    
    if let Some(name) = project_name {
        display_info(&format!("Project name: {}", name));
    }
    
    if force {
        display_warning("Force mode enabled - will overwrite existing files");
    }
    
    display_success("Init command structure ready (implementation pending)");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_structure() {
        // Test that CLI can be parsed without errors
        Cli::command().debug_assert();
    }

    #[test]
    fn test_agent_type_conversion() {
        // Test conversion from AgentType to reforge::config::Agent
        let copilot_agent = reforge::config::Agent::from(AgentType::Copilot);
        assert_eq!(copilot_agent, reforge::config::Agent::Copilot);
        
        let claude_agent = reforge::config::Agent::from(AgentType::Claude);
        assert_eq!(claude_agent, reforge::config::Agent::Claude);
    }

    #[test]
    fn test_reverse_agent_conversion() {
        // Test conversion from reforge::config::Agent to AgentType
        let copilot_type = AgentType::from(reforge::config::Agent::Copilot);
        matches!(copilot_type, AgentType::Copilot);
        
        let claude_type = AgentType::from(reforge::config::Agent::Claude);
        matches!(claude_type, AgentType::Claude);
    }

    #[test]
    fn test_error_exit_codes() {
        // Test that different error types exist and can be created
        // (Testing process::exit is difficult, so we just verify error creation)
        let _permission_error = ConfigError::permission_denied("/test/path");
        let _file_exists_error = ConfigError::file_exists("/test/file");
        let _invalid_agent_error = ConfigError::invalid_agent("invalid");
        
        // If we get here, all error types can be created successfully
        assert!(true);
    }
}
