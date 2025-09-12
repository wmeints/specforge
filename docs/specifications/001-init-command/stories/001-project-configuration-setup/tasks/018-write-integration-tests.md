# Task: Write Integration Tests Using assert_cmd

## Description
Create comprehensive integration tests for the init command using the assert_cmd crate as specified in CLAUDE.md.

## Required Files
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/STORY.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/RESEARCH.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/PLAN.md`

## Implementation Details
- Create `tests/integration_init.rs` file
- Test `reforge init` with interactive agent selection
- Test `reforge init --agent copilot` direct specification
- Test `reforge init --agent claude` direct specification
- Test `reforge init --output-directory <path>` custom directory
- Test overwrite confirmation for existing files
- Test invalid agent error handling
- Test permission denied scenarios
- Verify `.reforge.json` file creation and content using assert_cmd

## Acceptance Criteria
- All CLI scenarios are covered by integration tests
- Tests verify both command success and output content
- File creation and content are validated after command execution
- Error scenarios produce appropriate exit codes and messages
- Tests use temporary directories to avoid interference
- Integration tests run successfully with `cargo test`
- Tests verify JSON schema compliance of generated files

## Phase
Phase 5: Testing