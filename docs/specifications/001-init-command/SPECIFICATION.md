# Feature specification

- Created: 2025-09-12
- Status: Ready for Review

---

## User scenarios and testing (required)

### Primary user story

As a developer, I want to initialize a new project with Reforge so that I can quickly set up prompt templates for my chosen AI coding agent (GitHub Copilot or Claude Code) and start following a specification-driven workflow.

### Acceptance Test Scenarios

1. Given I'm in an empty directory, When I run `reforge init`, Then the system prompts me to choose an agent (copilot or claude) and initializes the project with the appropriate templates
2. Given I'm in an empty directory, When I run `reforge init --agent copilot`, Then the system initializes the project with GitHub Copilot templates without prompting
3. Given I'm in an empty directory, When I run `reforge init --agent claude`, Then the system initializes the project with Claude Code templates without prompting
4. Given I want to initialize in a specific directory, When I run `reforge init --output-directory ./my-project`, Then the system creates the directory and initializes the project there
5. Given a directory with existing files, When I run `reforge init`, Then the system asks for confirmation before overwriting any files
6. Given I confirm overwriting, When the system proceeds with initialization, Then existing files are replaced with template files

### Error handling (required)

1. Invalid agent specified: System displays error message listing valid agents (copilot, claude)
2. Permission denied: System displays clear error when lacking write permissions to target directory
3. User declines overwrite confirmation: System exits gracefully without making changes
4. Network/template access issues: System displays helpful error message if templates cannot be accessed

### Edge cases (required)

1. Target directory doesn't exist: System creates the directory structure
2. Partial existing project: System handles mixed scenarios where some template files exist and others don't
3. Read-only files in target directory: System handles permission conflicts appropriately
4. Existing git repository: System initializes in directories that already contain git repositories without interacting with git

## Functional requirements (required)

1. System MUST provide an `init` command that initializes a new Reforge project
2. System MUST support two agents: "copilot" for GitHub Copilot and "claude" for Claude Code
3. System MUST prompt for agent selection when no `--agent` flag is provided
4. System MUST accept `--agent` flag with values "copilot" or "claude"
5. System MUST initialize in current working directory by default
6. System MUST support `--output-directory` flag to specify target directory
7. System MUST create target directory if it doesn't exist
8. System MUST ask for confirmation before overwriting existing files
9. System MUST deploy appropriate prompt templates based on selected agent
10. System MUST create a `.reforge.json` file containing the deployed package version and agent configuration
11. System MUST NOT interact with git or any version control system
12. System SHOULD provide clear feedback about what files are being created/modified

## Non-functional requirements (required)

1. Performance: Initialization should complete within 5 seconds for typical projects
2. Cross-platform: Must work on Linux, Mac, and Windows as per project constraints
3. User experience: Clear, intuitive prompts with helpful error messages
4. Reliability: Graceful handling of filesystem errors and edge cases

## Data entities (include if feature works with data)

1. **Project Configuration**: `.reforge.json` file containing agent type, deployed package version, and project metadata
2. **Template Files**: Prompt template files specific to the chosen agent (copilot vs claude)
3. **Directory Structure**: Standard directory layout for Reforge projects

Note: The template package contains templates for all supported agent types, but only templates for the selected agent are deployed during initialization.

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
- [x] Dependencies and assumptions are documented

Quality Score: 5/5
Completeness Score: 5/5