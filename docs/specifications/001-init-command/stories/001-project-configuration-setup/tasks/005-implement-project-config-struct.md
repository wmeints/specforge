# Task: Implement ProjectConfig Struct

## Description
Create the `ProjectConfig` struct in `src/config/project.rs` with serde traits for JSON serialization and configuration management.

## Required Files
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/STORY.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/RESEARCH.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/PLAN.md`

## Implementation Details
- Define `ProjectConfig` struct with serde derive macros
- Include fields: `agent` (Agent enum), `packages` (Vec<Package>), `metadata` (HashMap)
- Define `Package` struct with fields: `id` (String), `url` (Option<String>), `version` (String)
- Implement serialization/deserialization methods
- Add validation methods for required fields
- Include constructor methods for creating new configurations

## Acceptance Criteria
- ProjectConfig and Package structs are properly defined
- JSON serialization produces the expected schema format
- Validation methods ensure required fields are present
- Constructor methods create valid default configurations
- All structs implement necessary serde traits

## Phase
Phase 2: Configuration Management