# Solution strategy

This section lists key decisions to address constraints and key requirements
in the project. It provides the foundational work needed for the building
blocks and runtime scenarios.

## Command-line utility

The Reforge application is implemented as Rust-based CLI so that it quick
to startup and compact. We choose to use
[Clap](https://docs.rs/clap/latest/clap/) to implement command-line argument
parsing for the tool.

## Storing template packs

The CLI doesn't contain template files. Instead we'll let the user configure
which template pack they want to use and where to get it from. We'll store
the configuration in `.reforge.json` in the users' home directory.

The configuration file will have a default for the template pack that the
tool should deploy to the developer's project.

The template pack is stored as a regular Zip archive. The content of the
template pack looks like this:

- `README.md` - An overview of the template pack
- `templates/copilot` - This directory contains Github Copilot templates
- `templates/claude` - This directory contains Claude templates

## Deploying template packs

When the user runs `reforge init --agent <agent-identifier>` we'll download
the configured template pack and unpack it in the appropriate directory within
the target directory. By default, when the user doesn't specify a directory
we'll use the current working directory as the root directory. The user can
change this by specifying an `--output-directory` option.

In the case of Github Copilot, we'll deploy the files from `templates/copilot`
in the pack archive into the directory `.github/prompts` directory under
the root directory.

For Claude Code we'll deploy the files from `templates/claude` in the pack
archive into the directory `.claude/commands` under the root directory.
