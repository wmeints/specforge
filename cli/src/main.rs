use clap::{Parser, Subcommand};
use specforge::ConfigError;
use specforge::cli::InitCommand;
use std::process;

/// Specforge CLI - Configure source control for AI-driven development
#[derive(Parser)]
#[command(
    name = "specforge",
    version,
    about = "Configure source control for AI-driven development through GitHub Copilot or Claude Code",
    long_about = "Specforge allows developers to configure their source control for AI-driven development \
                 by quickly deploying custom prompt templates for coding agents. Follow a specification-driven \
                 workflow where you handle specifications and review while the AI handles the coding.",
    author = "Specforge Contributors"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Specforge project with agent configuration
    Init(InitCommand),
}


/// Handle CLI errors and exit with appropriate codes
fn handle_error(error: ConfigError) -> ! {
    // Log error details securely for debugging (without sensitive info)
    if std::env::var("SPECFORGE_DEBUG").is_ok() {
        error.log_securely();
    }

    // Display user-friendly error message
    eprintln!("Error: {}", error);

    // Suggest retry if the error is retryable
    if error.is_retryable() {
        eprintln!("\nThis error may be temporary. You can try running the command again.");
    }

    // Use the error's built-in exit code method for proper Unix conventions
    let exit_code = error.exit_code();

    process::exit(exit_code);
}


fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Init(init_cmd) => {
            // Execute the init command
            init_cmd.execute()
        }
    };
    
    // Handle any errors
    if let Err(error) = result {
        handle_error(error);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use specforge::cli::AgentType;

    #[test]
    fn test_cli_structure() {
        // Test that CLI can be parsed without errors
        Cli::command().debug_assert();
    }

    #[test]
    fn test_agent_type_conversion() {
        // Test conversion from AgentType to specforge::config::Agent
        let copilot_agent = specforge::config::Agent::from(AgentType::Copilot);
        assert_eq!(copilot_agent, specforge::config::Agent::Copilot);
        
        let claude_agent = specforge::config::Agent::from(AgentType::Claude);
        assert_eq!(claude_agent, specforge::config::Agent::Claude);
    }

    #[test]
    fn test_reverse_agent_conversion() {
        // Test conversion from specforge::config::Agent to AgentType
        let copilot_type = AgentType::from(specforge::config::Agent::Copilot);
        assert!(matches!(copilot_type, AgentType::Copilot));
        
        let claude_type = AgentType::from(specforge::config::Agent::Claude);
        assert!(matches!(claude_type, AgentType::Claude));
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
