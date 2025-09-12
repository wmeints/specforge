# User story specification

Status: Ready for Development
Date: 2025-09-12
Priority: High
---

## Story description

As a developer, I want to run `reforge init` in an empty directory so that the system prompts me to choose an agent (copilot or claude) and initializes the project with the appropriate templates for specification-driven development.

## Acceptance criteria

- Running `reforge init` in empty directory prompts for agent selection
- Agent selection offers "copilot" and "claude" options
- System validates agent selection input
- System deploys templates specific to chosen agent
- System creates `.reforge.json` configuration file
- System provides clear feedback about files being created
- Initialization completes within 5 seconds

## Related Feature

Feature 001: Init Command - Initialize new Reforge projects

---

## Technical context

### Impact on the application architecture (optional)

- Implements core CLI command structure
- Establishes user interaction patterns
- Integrates template deployment system
- Uses configuration management from previous story

### Impact on the application functionality (optional)

- Provides primary entry point for new users
- Establishes standard project structure
- Creates foundation for other Reforge commands

### Impact on deployment of the application (optional)

No direct deployment impact

--- 

## Implementation tasks (required)

- Implement `init` subcommand in CLI framework
- Create interactive agent selection prompt
- Implement template deployment logic for each agent type
- Add progress feedback during initialization
- Integrate configuration file creation
- Implement directory validation (empty check)

## Testing tasks (required)

- Integration test full init flow for copilot agent
- Integration test full init flow for claude agent
- Test invalid agent input handling
- Test user cancellation during prompt
- Test timing requirements (< 5 seconds)
- Test file creation feedback messages

## Deployment tasks (required)

No specific deployment tasks required

---

## Dependencies (required)

- Story 001: Project Configuration Setup (requires config file creation)

## Assumptions (required)

- Users understand the difference between copilot and claude agents
- Terminal supports interactive prompts
- Template files are accessible during runtime
- Empty directory check is sufficient for safe initialization

[QUESTION: What should happen if user provides invalid input during agent selection? Should system re-prompt or exit?]