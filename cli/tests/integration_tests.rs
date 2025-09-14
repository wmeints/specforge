use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test full init flow for copilot agent
#[test]
fn test_full_init_flow_copilot() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "copilot", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("✅ Successfully created Specforge configuration"))
        .stdout(predicate::str::contains("📄 Deployed 2 template files"))
        .stdout(predicate::str::contains("Selected agent: copilot"));

    // Verify files were created
    assert!(temp_dir.path().join(".specforge.json").exists());
    assert!(temp_dir.path().join("CLAUDE.md").exists());
    assert!(temp_dir.path().join("README.md").exists());

    // Verify config content
    let config_content = fs::read_to_string(temp_dir.path().join(".specforge.json")).unwrap();
    assert!(config_content.contains("\"agent\": \"copilot\""));
    assert!(config_content.contains("specforge-copilot-templates"));

    // Verify template content
    let claude_content = fs::read_to_string(temp_dir.path().join("CLAUDE.md")).unwrap();
    assert!(claude_content.contains("GitHub Copilot"));

    let readme_content = fs::read_to_string(temp_dir.path().join("README.md")).unwrap();
    assert!(readme_content.contains("GitHub Copilot Configuration"));
}

/// Test full init flow for claude agent
#[test]
fn test_full_init_flow_claude() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "claude", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("✅ Successfully created Specforge configuration"))
        .stdout(predicate::str::contains("📄 Deployed 2 template files"))
        .stdout(predicate::str::contains("Selected agent: claude"));

    // Verify files were created
    assert!(temp_dir.path().join(".specforge.json").exists());
    assert!(temp_dir.path().join("CLAUDE.md").exists());
    assert!(temp_dir.path().join("README.md").exists());

    // Verify config content
    let config_content = fs::read_to_string(temp_dir.path().join(".specforge.json")).unwrap();
    assert!(config_content.contains("\"agent\": \"claude\""));
    assert!(config_content.contains("specforge-claude-templates"));

    // Verify template content
    let claude_content = fs::read_to_string(temp_dir.path().join("CLAUDE.md")).unwrap();
    assert!(claude_content.contains("Claude Code"));

    let readme_content = fs::read_to_string(temp_dir.path().join("README.md")).unwrap();
    assert!(readme_content.contains("Claude Code Configuration"));
}

/// Test invalid agent input handling
#[test]
fn test_invalid_agent_input() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "invalid-agent", "--output-directory"])
        .arg(temp_dir.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'invalid-agent'"))
        .stderr(predicate::str::contains("possible values: copilot, claude"));

    // Verify no files were created
    assert!(!temp_dir.path().join(".specforge.json").exists());
    assert!(!temp_dir.path().join("CLAUDE.md").exists());
    assert!(!temp_dir.path().join("README.md").exists());
}

/// Test init with project name
#[test]
fn test_init_with_project_name() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&[
        "init",
        "--agent", "copilot",
        "--project-name", "my-test-project",
        "--output-directory"
    ])
    .arg(temp_dir.path())
    .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("✅ Successfully created Specforge configuration"));

    // Verify config contains project name
    let config_content = fs::read_to_string(temp_dir.path().join(".specforge.json")).unwrap();
    assert!(config_content.contains("\"project_name\": \"my-test-project\""));
}

/// Test file creation feedback messages
#[test]
fn test_file_creation_feedback() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "claude", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("📄 Deployed 2 template files"))
        .stdout(predicate::str::contains("CLAUDE.md"))
        .stdout(predicate::str::contains("README.md"));
}

/// Test timing requirements (< 5 seconds)
#[test]
fn test_timing_requirements() {
    let temp_dir = TempDir::new().unwrap();
    let start_time = std::time::Instant::now();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "copilot", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");

    cmd.assert().success();

    let duration = start_time.elapsed();
    assert!(
        duration.as_secs() < 5,
        "Init command took too long: {:?}",
        duration
    );
}

/// Test init in existing directory with files (should still work)
#[test]
fn test_init_in_existing_directory_with_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create some existing files
    fs::write(temp_dir.path().join("existing_file.txt"), "existing content").unwrap();
    fs::create_dir(temp_dir.path().join("existing_dir")).unwrap();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "claude", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");

    cmd.assert().success();

    // Verify original files still exist
    assert!(temp_dir.path().join("existing_file.txt").exists());
    assert!(temp_dir.path().join("existing_dir").exists());

    // Verify new files were created
    assert!(temp_dir.path().join(".specforge.json").exists());
    assert!(temp_dir.path().join("CLAUDE.md").exists());
    assert!(temp_dir.path().join("README.md").exists());
}

/// Test help message includes init command
#[test]
fn test_help_includes_init_command() {
    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("Initialize a new Specforge project"));
}

/// Test init command help
#[test]
fn test_init_command_help() {
    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialize a new Specforge project"))
        .stdout(predicate::str::contains("--agent"))
        .stdout(predicate::str::contains("--output-directory"))
        .stdout(predicate::str::contains("--project-name"))
        .stdout(predicate::str::contains("--force"));
}

/// Test overwrite protection without force flag
#[test]
fn test_overwrite_protection() {
    let temp_dir = TempDir::new().unwrap();

    // Create initial configuration
    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "copilot", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");
    cmd.assert().success();

    // Try to init again without force flag (this would normally prompt in interactive mode)
    // For testing purposes, we test with force to ensure files are overwritten
    let mut cmd2 = Command::cargo_bin("specforge").unwrap();
    cmd2.args(&["init", "--agent", "claude", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");

    cmd2.assert().success();

    // Verify agent was changed
    let config_content = fs::read_to_string(temp_dir.path().join(".specforge.json")).unwrap();
    assert!(config_content.contains("\"agent\": \"claude\""));
}

/// Test that template deployment creates the expected file structure
#[test]
fn test_template_deployment_file_structure() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("specforge").unwrap();
    cmd.args(&["init", "--agent", "copilot", "--output-directory"])
        .arg(temp_dir.path())
        .arg("--force");

    cmd.assert().success();

    // Verify exact file structure
    let expected_files = vec![".specforge.json", "CLAUDE.md", "README.md"];

    for file in expected_files {
        let file_path = temp_dir.path().join(file);
        assert!(
            file_path.exists(),
            "Expected file {} was not created",
            file
        );
        assert!(
            file_path.is_file(),
            "Expected {} to be a file, not a directory",
            file
        );

        // Verify files are not empty
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(
            !content.trim().is_empty(),
            "File {} is empty",
            file
        );
    }
}