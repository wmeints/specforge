# Feature specification

- Created: 2025-09-11
- Status: Draft

## Input

Allow users to start a new project with `reforge init`. Ask the user for the agent (claude, or copilot) for which they want to initialize the project. When the user specifies `--agent (claude|copilot)` you shouldn't ask for the agent, but use the value provided. When the user didn't provide an `--output-directory` you should initialize in the current working directory. Otherwise, use the target directory.

---

## User scenarios and testing (required)

### Primary user story

As a developer starting a new software project, I want to quickly initialize my project with the appropriate prompt templates for my chosen AI coding agent (Claude Code or GitHub Copilot), so that I can immediately start using a specification-driven development workflow where the AI agent handles the coding while I focus on specifications and review.

### Acceptance Test Scenarios

1. Given a developer is in an empty directory, When they run `reforge init` and select "claude" when prompted, Then the system creates a `.claude/` directory with Claude-specific prompt templates in the current directory
2. Given a developer wants to use GitHub Copilot, When they run `reforge init --agent copilot`, Then the system creates a `.github/copilot/` directory with Copilot-specific templates without prompting for agent selection
3. Given a developer wants to initialize in a specific location, When they run `reforge init --output-directory ./my-project`, Then the system creates the appropriate agent configuration in the `./my-project` directory
4. Given a developer runs `reforge init` in a directory that already has agent configuration, When the command executes, Then the system prompts for confirmation to overwrite existing templates
5. Given a developer runs `reforge init --force` with existing templates, When the command executes, Then the system overwrites existing templates without prompting for confirmation
6. Given a developer runs `reforge init --agent invalid-agent`, When the command executes, Then the system displays an error message listing valid agent options (claude, copilot)

### Edge cases

- When the target directory doesn't exist, the system creates it automatically before initializing the project
- When the system cannot write to the target directory due to permission errors, it raises an error with a clear message
- When the user cancels any interactive prompt (agent selection or overwrite confirmation), the system stops the initialization process

## Functional requirements (required)

1. System MUST provide a `reforge init` command that initializes a project with prompt templates
2. System MUST support initialization for Claude Code agent with appropriate directory structure (`.claude/`)
3. System MUST support initialization for GitHub Copilot agent with appropriate directory structure (`.github/copilot/`)
4. System MUST accept an optional `--agent` flag to specify the agent type without interactive prompting
5. System MUST accept an optional `--output-directory` flag to specify the target initialization directory
6. System MUST accept an optional `--force` flag to overwrite existing templates without prompting
7. System MUST use the current working directory when no `--output-directory` is provided
8. System MUST only deploy templates for the single agent specified (not both agents)
9. System MUST prompt for confirmation before overwriting existing templates unless `--force` flag is provided
10. System SHOULD prompt the user interactively for agent selection when `--agent` flag is not provided
11. System SHOULD deploy default prompt templates (`/spec`, `/plan`, `/tasks`) appropriate for the selected agent
12. System SHOULD validate that the specified agent is supported (claude or copilot)
13. System SHOULD provide clear feedback about what files and directories were created

## Non-functional requirements (required)

1. The initialization process should complete within 2 seconds on standard hardware
2. The command should provide clear, actionable error messages when initialization fails
3. The deployed templates should be readable and editable by the developer post-initialization
4. The command should work consistently across Linux, Mac, and Windows platforms
5. File permissions for created directories and files should follow platform conventions (readable/writable by owner)

## Data entities (include if feature works with data)

1. **Agent Configuration**: Represents the selected AI agent type (claude or copilot) and its associated template directory structure
2. **Prompt Template**: Individual template files (spec, plan, tasks) that contain the prompts for the AI agent workflow

---

## Review checklist

### Content quality

- [x] Feature description contains no implementation details
- [x] Feature description focuses on business needs
- [x] Feature description is written for business stakeholders
- [x] All required sections are filled

### Completeness

- [x] Functional requirements are testable
- [x] Acceptance criteria are measurable
- [x] Scope is clearly defined
- [ ] Dependencies and assumptions are documented

Quality Score: 4/5
Completeness Score: 4/5
