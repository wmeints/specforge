use crate::config::Agent;
use crate::error::{ConfigError, Result};
use std::path::{Path, PathBuf};
use std::fs;

pub mod copilot;
pub mod claude;

/// Trait for deploying agent-specific templates
pub trait TemplateDeployer {
    /// Deploy templates for the given agent to the target directory
    fn deploy_templates(agent: &Agent, target_dir: &Path) -> Result<Vec<PathBuf>>;

    /// List template files for the given agent
    fn list_template_files(agent: &Agent) -> Vec<&'static str>;
}

/// Main template deployment implementation
pub struct TemplateSystem;

impl TemplateDeployer for TemplateSystem {
    fn deploy_templates(agent: &Agent, target_dir: &Path) -> Result<Vec<PathBuf>> {
        // Ensure target directory exists
        if !target_dir.exists() {
            fs::create_dir_all(target_dir).map_err(|e| {
                ConfigError::directory_creation_failed(target_dir, e)
            })?;
        }

        // Validate target directory is actually a directory
        if !target_dir.is_dir() {
            return Err(ConfigError::validation_error(format!(
                "Target path '{}' exists but is not a directory",
                target_dir.display()
            )));
        }

        let mut deployed_files = Vec::new();

        match agent {
            Agent::Copilot => {
                deployed_files.extend(copilot::deploy_copilot_templates(target_dir)?);
            }
            Agent::Claude => {
                deployed_files.extend(claude::deploy_claude_templates(target_dir)?);
            }
        }

        Ok(deployed_files)
    }

    fn list_template_files(agent: &Agent) -> Vec<&'static str> {
        match agent {
            Agent::Copilot => copilot::list_copilot_templates(),
            Agent::Claude => claude::list_claude_templates(),
        }
    }
}

/// Deploy template content to a file, handling existing files appropriately
fn deploy_template_file(
    content: &str,
    target_path: &Path,
    file_name: &str,
) -> Result<PathBuf> {
    let file_path = target_path.join(file_name);

    // Check if file already exists
    if file_path.exists() {
        // For now, we'll overwrite existing template files
        // This behavior could be made configurable in the future
        println!("⚠️  Overwriting existing file: {}", file_path.display());
    }

    // Write the template content
    fs::write(&file_path, content).map_err(|e| {
        match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                ConfigError::permission_denied(&file_path)
            }
            _ => ConfigError::io_error(format!(
                "Failed to write template file '{}': {}",
                file_path.display(),
                e
            ))
        }
    })?;

    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_deploy_templates_copilot() {
        let temp_dir = TempDir::new().unwrap();

        let deployed = TemplateSystem::deploy_templates(&Agent::Copilot, temp_dir.path()).unwrap();

        assert!(!deployed.is_empty());
        for file_path in &deployed {
            assert!(file_path.exists());
            assert!(file_path.is_file());
        }

        // Check that CLAUDE.md and README.md exist
        assert!(temp_dir.path().join("CLAUDE.md").exists());
        assert!(temp_dir.path().join("README.md").exists());
    }

    #[test]
    fn test_deploy_templates_claude() {
        let temp_dir = TempDir::new().unwrap();

        let deployed = TemplateSystem::deploy_templates(&Agent::Claude, temp_dir.path()).unwrap();

        assert!(!deployed.is_empty());
        for file_path in &deployed {
            assert!(file_path.exists());
            assert!(file_path.is_file());
        }

        // Check that CLAUDE.md and README.md exist
        assert!(temp_dir.path().join("CLAUDE.md").exists());
        assert!(temp_dir.path().join("README.md").exists());
    }

    #[test]
    fn test_list_template_files() {
        let copilot_templates = TemplateSystem::list_template_files(&Agent::Copilot);
        assert!(!copilot_templates.is_empty());
        assert!(copilot_templates.contains(&"CLAUDE.md"));
        assert!(copilot_templates.contains(&"README.md"));

        let claude_templates = TemplateSystem::list_template_files(&Agent::Claude);
        assert!(!claude_templates.is_empty());
        assert!(claude_templates.contains(&"CLAUDE.md"));
        assert!(claude_templates.contains(&"README.md"));
    }

    #[test]
    fn test_deploy_templates_nonexistent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_dir = temp_dir.path().join("nonexistent");

        // Should create directory and succeed
        let result = TemplateSystem::deploy_templates(&Agent::Copilot, &nonexistent_dir);
        assert!(result.is_ok());
        assert!(nonexistent_dir.exists());
        assert!(nonexistent_dir.is_dir());
    }

    #[test]
    fn test_deploy_templates_file_conflict() {
        use std::fs;
        let temp_dir = TempDir::new().unwrap();

        // Create a regular file at the target path
        let file_path = temp_dir.path().join("not_a_dir");
        fs::write(&file_path, "test content").unwrap();

        let result = TemplateSystem::deploy_templates(&Agent::Copilot, &file_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not a directory"));
    }

    #[test]
    fn test_deploy_template_file_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let existing_file = temp_dir.path().join("test.txt");

        // Create existing file
        fs::write(&existing_file, "old content").unwrap();

        // Deploy new content
        let result = deploy_template_file("new content", temp_dir.path(), "test.txt");
        assert!(result.is_ok());

        // Verify content was overwritten
        let content = fs::read_to_string(&existing_file).unwrap();
        assert_eq!(content, "new content");
    }
}