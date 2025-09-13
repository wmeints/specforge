use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper function to create a command for testing
fn reforge_cmd() -> Command {
    Command::cargo_bin("reforge").unwrap()
}

/// Helper function to validate JSON file content
fn validate_json_content(file_path: &Path, expected_agent: &str) {
    assert!(file_path.exists(), "Configuration file should exist");

    let content = fs::read_to_string(file_path).expect("Should read config file");
    let json: serde_json::Value = serde_json::from_str(&content).expect("Should parse JSON");

    // Validate basic structure
    assert!(json.get("agent").is_some(), "Should have agent field");
    assert!(json.get("packages").is_some(), "Should have packages field");
    assert!(json.get("metadata").is_some(), "Should have metadata field");

    // Validate agent value
    assert_eq!(
        json.get("agent").unwrap().as_str().unwrap(),
        expected_agent,
        "Agent should match expected value"
    );

    // Validate packages array
    let packages = json.get("packages").unwrap().as_array().unwrap();
    assert_eq!(packages.len(), 1, "Should have one default package");

    let package = &packages[0];
    assert!(package.get("id").is_some(), "Package should have id");
    assert!(package.get("version").is_some(), "Package should have version");
    assert!(package.get("url").is_some(), "Package should have url field");

    // Validate metadata
    let metadata = json.get("metadata").unwrap().as_object().unwrap();
    assert!(metadata.contains_key("created_at"), "Should have created_at timestamp");
    assert!(metadata.contains_key("initialized_by"), "Should have initialized_by field");
    assert!(metadata.contains_key("version"), "Should have version field");
}

#[test]
fn test_init_with_copilot_agent() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing Reforge project"))
        .stdout(predicate::str::contains("Selected agent: copilot"))
        .stdout(predicate::str::contains("Successfully created Reforge configuration"));

    // Validate file creation and content
    let config_path = temp_dir.path().join(".reforge.json");
    validate_json_content(&config_path, "copilot");
}

#[test]
fn test_init_with_claude_agent() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("claude")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing Reforge project"))
        .stdout(predicate::str::contains("Selected agent: claude"))
        .stdout(predicate::str::contains("Successfully created Reforge configuration"));

    // Validate file creation and content
    let config_path = temp_dir.path().join(".reforge.json");
    validate_json_content(&config_path, "claude");
}

#[test]
fn test_init_with_project_name() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--project-name")
        .arg("my-test-project")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully created Reforge configuration"));

    // Validate project name in JSON
    let config_path = temp_dir.path().join(".reforge.json");
    let content = fs::read_to_string(&config_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    let metadata = json.get("metadata").unwrap().as_object().unwrap();
    assert_eq!(
        metadata.get("project_name").unwrap().as_str().unwrap(),
        "my-test-project"
    );
}

#[test]
fn test_init_to_current_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Change to temp directory and run init without output-directory
    reforge_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--agent")
        .arg("claude")
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully created Reforge configuration"));

    // Validate file creation in current directory
    let config_path = temp_dir.path().join(".reforge.json");
    validate_json_content(&config_path, "claude");
}

#[test]
fn test_init_with_force_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".reforge.json");

    // Create initial config
    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success();

    // Verify initial config
    validate_json_content(&config_path, "copilot");

    // Overwrite with force flag
    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("claude")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully created Reforge configuration"));

    // Verify config was overwritten
    validate_json_content(&config_path, "claude");
}

#[test]
fn test_init_invalid_agent() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("invalid-agent")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .code(2) // clap validation error
        .stderr(predicate::str::contains("invalid value"));

    // Verify no config file was created
    let config_path = temp_dir.path().join(".reforge.json");
    assert!(!config_path.exists(), "Config file should not be created on error");
}

#[test]
fn test_init_nonexistent_directory() {
    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--output-directory")
        .arg("/this/path/does/not/exist/definitely")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:").or(predicate::str::contains("Error")));
    // Note: Exit code may vary depending on the specific error
}

#[test]
fn test_init_missing_agent_flag() {
    let temp_dir = TempDir::new().unwrap();

    // This should fail because agent is required
    // The actual behavior depends on the implementation
    reforge_cmd()
        .arg("init")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .timeout(std::time::Duration::from_secs(5)) // Prevent hanging
        .assert()
        .failure()
        .stderr(predicate::str::contains("required").or(predicate::str::contains("agent")));
}

#[test]
fn test_init_help_message() {
    reforge_cmd()
        .arg("init")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialize a new Reforge project"))
        .stdout(predicate::str::contains("--agent"))
        .stdout(predicate::str::contains("--output-directory"))
        .stdout(predicate::str::contains("--project-name"))
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn test_init_creates_directory_if_needed() {
    let temp_dir = TempDir::new().unwrap();
    let nested_dir = temp_dir.path().join("new").join("nested").join("directory");

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--output-directory")
        .arg(&nested_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Creating output directory"))
        .stdout(predicate::str::contains("Successfully created Reforge configuration"));

    // Verify directory and file were created
    assert!(nested_dir.exists(), "Nested directory should be created");
    assert!(nested_dir.is_dir(), "Path should be a directory");

    let config_path = nested_dir.join(".reforge.json");
    validate_json_content(&config_path, "copilot");
}

#[test]
fn test_json_schema_compliance() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("claude")
        .arg("--project-name")
        .arg("schema-test")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success();

    let config_path = temp_dir.path().join(".reforge.json");
    let content = fs::read_to_string(&config_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Validate JSON schema compliance
    assert!(json.is_object(), "Root should be an object");

    // Required fields
    assert!(json.get("agent").is_some(), "Missing required field: agent");
    assert!(json.get("packages").is_some(), "Missing required field: packages");
    assert!(json.get("metadata").is_some(), "Missing required field: metadata");

    // Field types
    assert!(json.get("agent").unwrap().is_string(), "Agent should be string");
    assert!(json.get("packages").unwrap().is_array(), "Packages should be array");
    assert!(json.get("metadata").unwrap().is_object(), "Metadata should be object");

    // Package structure
    let packages = json.get("packages").unwrap().as_array().unwrap();
    for package in packages {
        assert!(package.get("id").unwrap().is_string(), "Package id should be string");
        assert!(package.get("version").unwrap().is_string(), "Package version should be string");
        // URL can be null or string
        let url = package.get("url").unwrap();
        assert!(url.is_null() || url.is_string(), "Package url should be null or string");
    }

    // Metadata structure
    let metadata = json.get("metadata").unwrap().as_object().unwrap();
    assert!(metadata.get("created_at").unwrap().is_string(), "created_at should be string");
    assert!(metadata.get("initialized_by").unwrap().is_string(), "initialized_by should be string");
    assert!(metadata.get("version").unwrap().is_string(), "version should be string");
}

#[test]
fn test_init_preserves_json_formatting() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success();

    let config_path = temp_dir.path().join(".reforge.json");
    let content = fs::read_to_string(&config_path).unwrap();

    // Verify JSON is pretty-printed
    assert!(content.contains('\n'), "JSON should contain newlines");
    assert!(content.contains("  "), "JSON should contain indentation");
    assert!(content.trim().starts_with('{'), "JSON should start with opening brace");

    // Verify it's valid JSON
    let _: serde_json::Value = serde_json::from_str(&content).unwrap();
}

#[test]
fn test_init_version_consistency() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("claude")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success();

    let config_path = temp_dir.path().join(".reforge.json");
    let content = fs::read_to_string(&config_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Get version from package and metadata
    let package_version = json.get("packages").unwrap()
        .as_array().unwrap()[0]
        .get("version").unwrap()
        .as_str().unwrap();

    let metadata_version = json.get("metadata").unwrap()
        .get("version").unwrap()
        .as_str().unwrap();

    // Versions should be consistent (both should use CARGO_PKG_VERSION)
    assert_eq!(package_version, metadata_version, "Package and metadata versions should match");

    // Version should follow semantic versioning format
    let version_parts: Vec<&str> = package_version.split('.').collect();
    assert!(version_parts.len() >= 3, "Version should have at least 3 parts");

    // Each part should be numeric
    for part in &version_parts[0..3] {
        assert!(part.parse::<u32>().is_ok(), "Version part should be numeric: {}", part);
    }
}

#[test]
fn test_init_package_id_format() {
    let temp_dir = TempDir::new().unwrap();

    // Test Copilot package ID
    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--output-directory")
        .arg(temp_dir.path().join("copilot"))
        .assert()
        .success();

    let copilot_config = temp_dir.path().join("copilot").join(".reforge.json");
    let copilot_content = fs::read_to_string(&copilot_config).unwrap();
    let copilot_json: serde_json::Value = serde_json::from_str(&copilot_content).unwrap();

    let copilot_package_id = copilot_json.get("packages").unwrap()
        .as_array().unwrap()[0]
        .get("id").unwrap()
        .as_str().unwrap();

    assert_eq!(copilot_package_id, "reforge-copilot-templates");

    // Test Claude package ID
    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("claude")
        .arg("--output-directory")
        .arg(temp_dir.path().join("claude"))
        .assert()
        .success();

    let claude_config = temp_dir.path().join("claude").join(".reforge.json");
    let claude_content = fs::read_to_string(&claude_config).unwrap();
    let claude_json: serde_json::Value = serde_json::from_str(&claude_content).unwrap();

    let claude_package_id = claude_json.get("packages").unwrap()
        .as_array().unwrap()[0]
        .get("id").unwrap()
        .as_str().unwrap();

    assert_eq!(claude_package_id, "reforge-claude-templates");

    // Package IDs should be different for different agents
    assert_ne!(copilot_package_id, claude_package_id);
}

#[test]
fn test_init_error_messages_quality() {
    // Test with permission denied scenario (if possible to simulate)
    // Note: This test might be platform-specific and may not work in all environments
    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("copilot")
        .arg("--output-directory")
        .arg("/root/no-permission") // Likely to fail with permission denied
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:").or(predicate::str::contains("Error")));
    // Note: We can't test specific error messages without knowing the OS/permission setup
}

#[test]
fn test_init_with_all_flags() {
    let temp_dir = TempDir::new().unwrap();

    reforge_cmd()
        .arg("init")
        .arg("--agent")
        .arg("claude")
        .arg("--project-name")
        .arg("comprehensive-test")
        .arg("--output-directory")
        .arg(temp_dir.path())
        .arg("--force") // Should work even without existing file
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing Reforge project"))
        .stdout(predicate::str::contains("Selected agent: claude"))
        .stdout(predicate::str::contains("Successfully created Reforge configuration"));

    // Validate all aspects of the generated config
    let config_path = temp_dir.path().join(".reforge.json");
    validate_json_content(&config_path, "claude");

    // Validate project name
    let content = fs::read_to_string(&config_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(
        json.get("metadata").unwrap()
            .get("project_name").unwrap()
            .as_str().unwrap(),
        "comprehensive-test"
    );
}