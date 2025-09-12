# Task: Verify .reforge.json File Format and Contents

## Description
Verify that the generated `.reforge.json` file matches the specified schema and contains all required information in the correct format.

## Required Files
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/STORY.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/RESEARCH.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/PLAN.md`

## Implementation Details
- Generate `.reforge.json` files with different agent selections
- Verify JSON structure matches the schema from research
- Confirm all required fields are present and correctly typed
- Validate agent field contains correct values (copilot or claude)
- Verify packages array structure and content
- Check metadata field contains creation timestamp and project information
- Validate JSON is properly formatted and human-readable
- Test file parsing with standard JSON parsers

## Acceptance Criteria
- Generated JSON matches the specified schema exactly
- Agent field correctly reflects the selected agent type
- Packages array contains appropriate template package information
- Metadata includes creation timestamp in ISO 8601 format
- JSON is properly indented and human-readable
- File can be successfully parsed by standard JSON libraries
- All required fields are present with correct data types
- File size is reasonable and efficient

## Phase
Phase 6: Validation