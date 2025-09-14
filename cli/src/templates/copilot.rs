use crate::error::Result;
use std::path::{Path, PathBuf};

/// GitHub Copilot template content
const COPILOT_CLAUDE_MD: &str = include_str!("../../templates/copilot/CLAUDE.md");
const COPILOT_README_MD: &str = include_str!("../../templates/copilot/README.md");

/// Deploy GitHub Copilot templates to the target directory
pub fn deploy_copilot_templates(target_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut deployed_files = Vec::new();

    // Deploy CLAUDE.md
    let claude_path = super::deploy_template_file(
        COPILOT_CLAUDE_MD,
        target_dir,
        "CLAUDE.md"
    )?;
    deployed_files.push(claude_path);

    // Deploy README.md
    let readme_path = super::deploy_template_file(
        COPILOT_README_MD,
        target_dir,
        "README.md"
    )?;
    deployed_files.push(readme_path);

    Ok(deployed_files)
}

/// List all GitHub Copilot template files
pub fn list_copilot_templates() -> Vec<&'static str> {
    vec![
        "CLAUDE.md",
        "README.md",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_deploy_copilot_templates() {
        let temp_dir = TempDir::new().unwrap();

        let deployed = deploy_copilot_templates(temp_dir.path()).unwrap();

        assert_eq!(deployed.len(), 2);

        // Verify files exist
        let claude_path = temp_dir.path().join("CLAUDE.md");
        let readme_path = temp_dir.path().join("README.md");

        assert!(claude_path.exists());
        assert!(readme_path.exists());

        // Verify file contents
        let claude_content = std::fs::read_to_string(&claude_path).unwrap();
        assert!(claude_content.contains("GitHub Copilot"));
        assert!(claude_content.contains("specforge"));

        let readme_content = std::fs::read_to_string(&readme_path).unwrap();
        assert!(readme_content.contains("GitHub Copilot Configuration"));
        assert!(readme_content.contains("Setup Instructions"));
    }

    #[test]
    fn test_list_copilot_templates() {
        let templates = list_copilot_templates();

        assert_eq!(templates.len(), 2);
        assert!(templates.contains(&"CLAUDE.md"));
        assert!(templates.contains(&"README.md"));
    }

    #[test]
    fn test_copilot_template_content() {
        // Verify the embedded template content is valid
        assert!(!COPILOT_CLAUDE_MD.is_empty());
        assert!(!COPILOT_README_MD.is_empty());

        // Verify content contains expected keywords
        assert!(COPILOT_CLAUDE_MD.contains("GitHub Copilot"));
        assert!(COPILOT_README_MD.contains("GitHub Copilot Configuration"));
    }
}