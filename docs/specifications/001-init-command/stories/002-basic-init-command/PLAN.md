# Implementation plan for Basic Init Command

Status: Ready for Implementation
Date: 2025-09-13

---

## Code changes (required)

Based on the research findings, the core CLI framework and configuration management are already well-implemented. However, the following changes are needed to complete the basic init command functionality:

### 1. Package-Based Template System Implementation

**Location:** `src/templates/` (new module)
- Create `src/templates/mod.rs` with package deployment logic
- Create `src/templates/package.rs` with package extraction and deployment
- Implement `PackageDeployer` trait with methods:
  - `deploy_package(agent: &Agent, target_dir: &Path) -> Result<Vec<PathBuf>>`
  - `extract_package_contents(agent: &Agent) -> Result<PackageContents>`
  - `validate_package_structure(contents: &PackageContents) -> Result<()>`

**Package Specification:**
- Packages are stored as ZIP containers embedded in the binary
- Each package contains two top-level directories: `copilot/` and `claude/`
- Each agent directory contains:
  - `PROMPTS.md` - Documentation explaining how to use the prompts for that agent
  - Agent-specific prompt template directories:
    - Claude Code: `.claude/commands/` directory structure
    - GitHub Copilot: `.github/prompts/` directory structure
- When deploying, only the selected agent's directory contents are extracted to the target directory, preserving the original structure

### 2. Integration with InitCommand

**Location:** `src/cli/init.rs` (existing file)
- Add package deployment step to the `execute()` method after configuration file creation
- Add progress feedback for package extraction and deployment process
- Handle package deployment failures gracefully with rollback capability
- Update success message to include information about deployed package contents

**Code Changes:**
```rust
// Add to InitCommand::execute() after line 161
let deployed_files = self.deploy_package(&agent, &self.output_directory)?;
println!("ℹ️  Deployed {} files from {} package", deployed_files.len(), agent.name());
for file in &deployed_files {
    println!("   📄 {}", file.display());
}
```

### 3. Package Directory Structure

**Create package structure in project:**
- `packages/default.zip` - Default package containing both agent templates
- Package contents structure:
  ```
  copilot/
  ├── PROMPTS.md - GitHub Copilot prompt usage documentation
  └── .github/
      └── prompts/ - GitHub Copilot prompt template directory
  claude/
  ├── PROMPTS.md - Claude Code prompt usage documentation
  └── .claude/
      └── commands/ - Claude Code prompt template directory
  ```

### 4. Enhanced Progress Feedback

**Location:** `src/cli/init.rs` (existing file)
- Add more granular progress messages during package extraction and deployment
- Implement timing feedback to ensure 5-second requirement compliance
- Add summary statistics (number of files created, total time taken)
- Show which agent-specific package contents are being deployed

---

## Test changes (required)

### 1. Unit Tests for Package System

**Location:** `src/templates/mod.rs`
- Test package structure validation and ZIP extraction
- Test package deployment to various directory structures
- Test agent-specific directory selection (copilot vs claude)
- Test rollback behavior on deployment failures
- Test file overwrite handling during package deployment
- Test package integrity and embedded ZIP validation

### 2. Integration Tests for Full Init Flow

**Location:** `tests/integration_tests.rs` (new file)
- Test complete init flow for copilot agent with package deployment
- Test complete init flow for claude agent with package deployment
- Test invalid agent input handling in full context
- Test user cancellation during different phases (agent selection, file overwrite, etc.)
- Test timing requirements (< 5 seconds) with actual package deployment
- Test file creation feedback messages accuracy
- Test that correct agent-specific files are deployed from package

### 3. Enhanced Unit Tests

**Location:** `src/cli/init.rs` (existing test module)
- Add tests for package deployment integration
- Test error handling when package deployment fails
- Test cleanup behavior on partial failures
- Test correct agent directory selection during deployment

**Test Cases to Add:**
```rust
#[test]
fn test_init_with_package_deployment() {
    // Test full init including package deployment
}

#[test]
fn test_init_package_deployment_failure_rollback() {
    // Test rollback when package deployment fails
}

#[test]
fn test_init_timing_requirements() {
    // Test that full init completes within 5 seconds
}

#[test]
fn test_init_agent_specific_package_deployment() {
    // Test that only correct agent directory is deployed
}
```

---

## Documentation changes (optional)

No architecture documentation changes are required as the existing design accommodates the package-based template system through embedded ZIP deployment.

**Optional Enhancement:** Update `docs/architecture/05-building-blocks-view.md` to document the new package deployment subsystem and ZIP container format if significant complexity is added.

---

## Task list

Tasks are ordered by their dependencies:

1. **Create package content structure**
   - Create `packages/default.zip` containing both agent directories
   - Create `copilot/PROMPTS.md` with GitHub Copilot prompt usage documentation
   - Create `copilot/.github/prompts/` directory structure with templates
   - Create `claude/PROMPTS.md` with Claude Code prompt usage documentation
   - Create `claude/.claude/commands/` directory structure with templates

2. **Implement package deployment system**
   - Create `src/templates/mod.rs` with core package deployment logic
   - Create `src/templates/package.rs` with ZIP extraction and validation
   - Implement `PackageDeployer` trait and deployment methods
   - Add logic for agent-specific directory selection and extraction

3. **Integrate package deployment with InitCommand**
   - Add package deployment step to `InitCommand::execute()`
   - Add progress feedback for package extraction and deployment
   - Add deployed file listing to success message
   - Handle package deployment errors with appropriate rollback

4. **Add comprehensive error handling**
   - Handle ZIP extraction and package deployment failures
   - Implement cleanup/rollback for partial deployments
   - Add specific error messages for package-related issues
   - Validate package structure integrity during deployment

5. **Implement unit tests for package system**
   - Test package structure validation and ZIP extraction
   - Test agent-specific directory selection and deployment
   - Test error conditions and rollback behavior
   - Test package integrity validation

6. **Add integration tests for full init flow**
   - Test complete init flow with package deployment for both agents
   - Test performance requirements (< 5 seconds)
   - Test user interaction scenarios (cancellation, confirmations)
   - Test file creation feedback accuracy
   - Test correct agent-specific file deployment

7. **Performance optimization and validation**
   - Profile full init flow to ensure < 5 second completion
   - Optimize package extraction and deployment if necessary
   - Add timing feedback in user interface

8. **Final integration testing and validation**
   - Test all acceptance criteria end-to-end
   - Validate error handling and user experience
   - Verify file creation feedback messages accuracy
   - Verify correct package structure deployment

---

## Review Checklist

### Technical Implementation
- [ ] Package deployment system integrates cleanly with existing InitCommand
- [ ] ZIP extraction and validation are secure and robust
- [ ] Error handling maintains existing robustness standards
- [ ] Performance meets 5-second requirement for typical usage
- [ ] User feedback maintains consistent style and clarity
- [ ] Agent-specific directory selection works correctly

### Code Quality
- [ ] Package system follows existing project patterns and conventions
- [ ] Error messages are helpful and actionable
- [ ] Code is properly documented with doc comments
- [ ] Integration maintains separation of concerns
- [ ] ZIP handling is memory-efficient and secure

### Testing Coverage
- [ ] All new code paths have corresponding unit tests
- [ ] Integration tests cover full user workflows
- [ ] Error conditions are thoroughly tested
- [ ] Performance requirements are validated through tests
- [ ] Package structure validation is comprehensive
- [ ] Agent-specific deployment scenarios are tested

### User Experience
- [ ] Progress feedback is clear and informative
- [ ] Error messages guide users toward resolution
- [ ] Success messages provide helpful next steps
- [ ] File creation feedback shows what was actually created
- [ ] Users understand which agent-specific files were deployed

### Questions for Implementation
- [QUESTION: What specific content should be included in the PROMPTS.md files for each agent?]
- [QUESTION: Should package deployment preserve file permissions from the ZIP archive?]
- [QUESTION: How should we handle conflicts when package files already exist in the target directory?]
- [QUESTION: Should we validate ZIP integrity or trust embedded packages?]
- [QUESTION: Should we add progress bars for package deployment or are status messages sufficient?]

**Quality Score:** 4/5 (high confidence in approach)
**Completeness Score:** 4/5 (requires template content decisions)
**Implementation Complexity:** Medium (leverages existing robust foundation)