use crate::error::Result;
use std::path::{Path, PathBuf};

/// Claude Code template content
const CLAUDE_CLAUDE_MD: &str = include_str!("../../templates/claude/CLAUDE.md");
const CLAUDE_README_MD: &str = include_str!("../../templates/claude/README.md");

/// Deploy Claude Code templates to the target directory
pub fn deploy_claude_templates(target_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut deployed_files = Vec::new();

    // Deploy CLAUDE.md
    let claude_path = super::deploy_template_file(
        CLAUDE_CLAUDE_MD,
        target_dir,
        "CLAUDE.md"
    )?;
    deployed_files.push(claude_path);

    // Deploy README.md
    let readme_path = super::deploy_template_file(
        CLAUDE_README_MD,
        target_dir,
        "README.md"
    )?;
    deployed_files.push(readme_path);

    Ok(deployed_files)
}

/// List all Claude Code template files
pub fn list_claude_templates() -> Vec<&'static str> {
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
    fn test_deploy_claude_templates() {
        let temp_dir = TempDir::new().unwrap();

        let deployed = deploy_claude_templates(temp_dir.path()).unwrap();

        assert_eq!(deployed.len(), 2);

        // Verify files exist
        let claude_path = temp_dir.path().join("CLAUDE.md");
        let readme_path = temp_dir.path().join("README.md");

        assert!(claude_path.exists());
        assert!(readme_path.exists());

        // Verify file contents
        let claude_content = std::fs::read_to_string(&claude_path).unwrap();
        assert!(claude_content.contains("Claude Code"));
        assert!(claude_content.contains("specforge"));

        let readme_content = std::fs::read_to_string(&readme_path).unwrap();
        assert!(readme_content.contains("Claude Code Configuration"));
        assert!(readme_content.contains("Setup Instructions"));
    }

    #[test]
    fn test_list_claude_templates() {
        let templates = list_claude_templates();

        assert_eq!(templates.len(), 2);
        assert!(templates.contains(&"CLAUDE.md"));
        assert!(templates.contains(&"README.md"));
    }

    #[test]
    fn test_claude_template_content() {
        // Verify the embedded template content is valid
        assert!(!CLAUDE_CLAUDE_MD.is_empty());
        assert!(!CLAUDE_README_MD.is_empty());

        // Verify content contains expected keywords
        assert!(CLAUDE_CLAUDE_MD.contains("Claude Code"));
        assert!(CLAUDE_README_MD.contains("Claude Code Configuration"));
    }
}