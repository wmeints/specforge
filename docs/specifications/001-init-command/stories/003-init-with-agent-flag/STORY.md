# User story specification

Status: Ready for Development
Date: 2025-09-12
Priority: Medium
---

## Story description

As a developer, I want to specify the agent directly via command line flag so that I can skip the interactive prompt and initialize projects in automated scripts or when I already know which agent I want to use.

## Acceptance criteria

- Running `reforge init --agent copilot` initializes with GitHub Copilot templates without prompting
- Running `reforge init --agent claude` initializes with Claude Code templates without prompting
- System validates agent flag values and rejects invalid agents
- System displays error message listing valid agents when invalid agent specified
- All other initialization behavior remains the same as interactive mode

## Related Feature

Feature 001: Init Command - Initialize new Reforge projects

---

## Technical context

### Impact on the application architecture (optional)

- Extends CLI argument parsing capabilities
- Adds validation for agent selection parameters
- Maintains consistency with interactive flow

### Impact on the application functionality (optional)

- Enables automation and scripting use cases
- Provides alternative to interactive user experience
- Supports power user workflows

### Impact on deployment of the application (optional)

No direct deployment impact

--- 

## Implementation tasks (required)

- Add `--agent` flag to init command argument parser
- Implement agent flag validation logic
- Create error messages for invalid agent values
- Bypass interactive prompt when flag provided
- Ensure consistent behavior with interactive mode
- Update command help text

## Testing tasks (required)

- Integration test `--agent copilot` initialization
- Integration test `--agent claude` initialization  
- Test invalid agent flag values (e.g., `--agent invalid`)
- Test case sensitivity of agent values
- Test that interactive prompt is skipped
- Test help text displays flag information

## Deployment tasks (required)

No specific deployment tasks required

---

## Dependencies (required)

- Story 001: Project Configuration Setup
- Story 002: Basic Init Command (extends this functionality)

## Assumptions (required)

- Command line argument parsing framework supports flags
- Agent names are case-sensitive
- Users prefer explicit error messages over generic validation failures
- Automated use cases are important for developer workflows