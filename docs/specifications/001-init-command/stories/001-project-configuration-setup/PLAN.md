# Implementation plan for Project Configuration Setup

Status: Ready for Implementation
Date: 2025-09-12

---

## Code changes (required)

### 1. Add Required Dependencies
Update `Cargo.toml` to include JSON serialization dependencies:
- Add `serde` with derive feature for struct serialization
- Add `serde_json` for JSON parsing and writing
- Add `dialoguer` for interactive CLI prompts (optional enhancement)

### 2. Create Configuration Module (`src/config/`)
Create a new configuration module with the following components:

**ProjectConfig Struct (`src/config/project.rs`)**:
- Define `ProjectConfig` struct with serde derive macros
- Include fields: `agent` (enum), `packages` (Vec<Package>), `metadata` (HashMap)
- Implement serialization/deserialization methods
- Add validation methods for required fields

**Package Struct (`src/config/project.rs`)**:
- Define `Package` struct with fields: `id` (String), `url` (Option<String>), `version` (String)
- Implement serde traits for JSON serialization
- Add validation for required fields

**Agent Enum (`src/config/project.rs`)**:
- Define `Agent` enum with variants `Copilot` and `Claude`
- Implement Display trait for string conversion
- Add FromStr trait for parsing from CLI arguments

### 3. Enhance CLI Structure (`src/cli/`)
Restructure the CLI to support the init command:

**Main CLI (`src/main.rs`)**:
- Replace placeholder with clap-based CLI structure
- Define main CLI struct with subcommands
- Add proper error handling and exit codes

**Init Command (`src/cli/init.rs`)**:
- Implement init subcommand with clap derive macros
- Support `--agent` flag for direct agent specification
- Support `--output-directory` flag for custom target directory
- Add interactive agent selection when no agent flag provided
- Implement confirmation prompt for existing file overwrite

### 4. File Operations Module (`src/lib.rs`)
Create utility functions for file system operations:
- Directory creation and validation
- JSON file writing with proper error handling
- Configuration file reading and validation
- Permission checking before file operations

### 5. Error Handling (`src/error.rs`)
Define custom error types for configuration management:
- ConfigError enum for configuration-related errors
- IoError wrapper for file system errors  
- JsonError wrapper for serialization errors
- User-friendly error messages with recovery suggestions

### 6. Configuration File Implementation
Implement `.reforge.json` file management:
- Create configuration file in target directory
- Include agent type (copilot or claude)
- Include packages array with deployed template packages (id, optional url, version)
- Include project metadata (creation timestamp, etc.)
- Ensure human-readable JSON formatting

## Test changes (required)

### 1. Unit Tests for Configuration Module
**Test file**: `src/config/project.rs` (inline tests)
- Test ProjectConfig struct creation and validation
- Test Agent enum parsing and display
- Test JSON serialization/deserialization  
- Test invalid data handling (missing fields, wrong types)
- Test packages array handling and validation

### 2. Unit Tests for File Operations
**Test file**: `src/lib.rs` (inline tests)
- Test directory creation and validation
- Test JSON file writing and reading
- Test permission error handling
- Test invalid JSON file recovery
- Use temporary directories for isolated testing

### 3. Integration Tests for Init Command
**Test file**: `tests/integration_init.rs`
- Test `reforge init` with interactive agent selection
- Test `reforge init --agent copilot` direct specification
- Test `reforge init --agent claude` direct specification  
- Test `reforge init --output-directory <path>` custom directory
- Test overwrite confirmation for existing files
- Test invalid agent error handling
- Test permission denied scenarios
- Verify `.reforge.json` file creation and content
- Use `assert_cmd` crate as specified in CLAUDE.md

### 4. Error Scenario Tests
**Test coverage for**:
- Invalid agent specification
- Permission denied on target directory
- User declines overwrite confirmation
- Invalid JSON in existing configuration files
- Read-only file system scenarios
- Non-existent parent directories

## Documentation changes (optional)

No architecture documentation changes required for this foundational story.

---

## Task list

### Phase 1: Foundation Setup
1. Update `Cargo.toml` with required dependencies (serde, serde_json)
2. Create project structure: `src/config/`, `src/cli/`, `src/error.rs` 
3. Define custom error types in `src/error.rs`

### Phase 2: Configuration Management
4. Implement `Agent` enum in `src/config/project.rs`
5. Implement `ProjectConfig` struct with serde traits in `src/config/project.rs`
6. Add configuration validation methods
7. Implement JSON file operations (read/write/validate)

### Phase 3: CLI Implementation  
8. Restructure `src/main.rs` with clap CLI framework
9. Implement init subcommand structure in `src/cli/init.rs`
10. Add agent selection logic (interactive and flag-based)
11. Add output directory handling with creation
12. Implement file overwrite confirmation

### Phase 4: Integration
13. Connect init command to configuration file creation
14. Add packages array creation with template package information
15. Implement graceful error handling throughout

### Phase 5: Testing
16. Write unit tests for configuration module
17. Write unit tests for file operations
18. Write integration tests using assert_cmd
19. Test error scenarios and edge cases
20. Verify all acceptance criteria are met

### Phase 6: Validation
21. Run `cargo test` to verify all tests pass
22. Run `cargo build` to ensure clean compilation
23. Manual testing of all CLI scenarios
24. Verify `.reforge.json` file format and contents