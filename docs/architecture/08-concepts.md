# Concepts

This section lists key concepts used throughout the project.

## Prompt template pack

The project uses prompt template packs as a means of grouping and packaging
prompt templates for use in software projects. We allow users to use their
own custom created template packs that fit their corporate standards.

Allowing developers to come up with their own template packages is important
as every organization may have different requirements. Also, we feel that
it could be useful to let the user overlay multiple packages at a later stage.

The structure of a template pack looks like this:

- `manifest.json` - Contains metadata for the template pack
- `README.md` - An overview of the template pack
- `templates/copilot` - This directory contains Github Copilot templates
- `templates/claude` - This directory contains Claude templates

We've included the README file so that the tool can render help information
for users when they want to read about a template pack first before deploying
it in the project.

Templates are stored in a directory per agent so that we can extend the support
of reforge to other coding agents in the future.
