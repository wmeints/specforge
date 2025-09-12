# Task: Implement Agent Enum

## Description
Create the `Agent` enum in `src/config/project.rs` to represent the different types of AI agents supported.

## Required Files
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/STORY.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/RESEARCH.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/PLAN.md`

## Implementation Details
- Define `Agent` enum with variants `Copilot` and `Claude`
- Implement `Display` trait for string conversion
- Add `FromStr` trait for parsing from CLI arguments
- Add serde derive macros for JSON serialization
- Include proper error handling for invalid agent names

## Acceptance Criteria
- Agent enum supports both `copilot` and `claude` variants
- String conversion works in both directions
- JSON serialization/deserialization works correctly
- Invalid agent names produce meaningful errors

## Phase
Phase 2: Configuration Management