# Instructions

This file provides guidance to Claude Code when working on the reforge
codebase.

## Project overview

This is the **Reforge** CLI project that allows developers to configure their
source control for AI-driven developments through Github Copilot or Claude
Code.

The goal of the project is to quickly deploy custom prompt templates for a
coding agent so that the developer can follow a specification driven workflow
where the developer is responsible for specifications and review and the
coding agent does all the coding.

### Development commands

- `cargo test` - Runs the automated tests for the CLI
- `cargo build` - Compiles the project into an executable
- `cargo run` - Compiles and runs the application in the
  development environment

### Project structure

The project has the following directory structure:

- `src` - Contains the source code for the CLI.
- `docs` - Contains product documentation, specs, and architecture
  documentation.
- `templates` - Contains the default set of prompt templates.

### Project architecture

- `docs/architecture/01-introduction.md` describes the primary requirements
  and quality goals for the project.
- `docs/architecture/02-constraints.md` describes the key constraints for the
  project.
- `docs/architecture/03-context-and-scope.md` describes the scope and context
  of the project.
- `docs/architecture/04-solution-strategy.md` describes the key solution
  strategy decisions.
- `docs/architecture/05-building-blocks-view.md` describes the core components
  of the project.
- `docs/architecture/06-runtime-scenarios.md` describes the key runtime
  scenarios of the project.
- `docs/architecture/07-deployment-view.md` describes the deployment approach
  to the project.
- `docs/architecture/08-concepts.md` describes the key concepts used in
  the project.

### Testing strategy

This project has two layers of tests:

- Unit-tests are co-located with the components being tested. See also:
  https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
- Integration tests are implemented using
  [assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/).

## Implementing tasks

When implementing a task file, please follow these guidelines:

1. Make sure you're on the feature branch before making any changes
2. Implement the task as instructed
3. Run the automated tests using the appropriate commands
