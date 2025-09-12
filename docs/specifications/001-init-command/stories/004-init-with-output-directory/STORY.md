# User story specification

Status: Ready for Development
Date: 2025-09-12
Priority: Medium
---

## Story description

As a developer, I want to specify a custom output directory for initialization so that I can create new Reforge projects in locations other than my current working directory.

## Acceptance criteria

- Running `reforge init --output-directory ./my-project` creates and initializes project in specified directory
- System creates target directory if it doesn't exist
- System creates full directory path including intermediate directories
- System handles both relative and absolute paths
- Agent selection and template deployment work normally in custom directory
- Configuration file is created in the specified directory

## Related Feature

Feature 001: Init Command - Initialize new Reforge projects

---

## Technical context

### Impact on the application architecture (optional)

- Extends CLI argument parsing for directory paths
- Adds directory creation and path handling logic
- Modifies initialization context to work with custom paths

### Impact on the application functionality (optional)

- Enables project creation in structured directory layouts
- Supports project organization workflows
- Allows initialization without changing current working directory

### Impact on deployment of the application (optional)

No direct deployment impact

--- 

## Implementation tasks (required)

- Add `--output-directory` flag to init command
- Implement directory path validation and normalization
- Add recursive directory creation functionality
- Modify initialization logic to work with custom directories
- Handle path resolution (relative vs absolute)
- Update all file operations to use specified directory

## Testing tasks (required)

- Test initialization in custom directory (relative path)
- Test initialization in custom directory (absolute path) 
- Test creation of non-existent directory
- Test creation of nested directory structure
- Test permission errors when creating directories
- Test invalid directory paths
- Verify all files created in correct location

## Deployment tasks (required)

No specific deployment tasks required

---

## Dependencies (required)

- Story 001: Project Configuration Setup
- Story 002: Basic Init Command (can be developed in parallel)

## Assumptions (required)

- File system supports directory creation operations
- Users understand relative vs absolute path concepts
- Directory permissions allow creation of new directories
- Path separator handling works across platforms (Linux, Mac, Windows)

[QUESTION: Should the system validate that the output directory name is suitable for a project directory, or accept any valid filesystem path?]