# Task: Add Agent Selection Logic

## Description
Implement agent selection logic in the init command that supports both interactive prompts and direct flag-based selection.

## Required Files
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/STORY.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/RESEARCH.md`
- `docs/specifications/001-init-command/stories/001-project-configuration-setup/PLAN.md`

## Implementation Details
- Add interactive agent selection when no `--agent` flag is provided
- Use dialoguer for interactive prompts with list selection
- Validate agent selection from command-line flags
- Provide clear options and descriptions for each agent type
- Handle user cancellation gracefully
- Convert selected agent to the Agent enum

## Acceptance Criteria
- Interactive prompt displays clear options for copilot and claude
- Direct agent specification via flag works correctly
- Invalid agent names produce helpful error messages
- User can cancel the interactive prompt
- Selected agent is properly converted to internal representation
- Selection logic is consistent between interactive and flag modes

## Phase
Phase 3: CLI Implementation