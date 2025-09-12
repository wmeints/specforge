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
}