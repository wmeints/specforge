# Context and scope

This section explains the context and scope of the Reforge application.

## Scope of the tool

The Reforge application is responsible for downloading and copying a specific
set of prompt templates into a directory that the user specifies. The tool
has no purpose beyond this task. It can also be used to upgrade already
deployed prompt templates in the project.

The developer will use either Claude Code or Github Copilot in combination
with the deployed prompt templates. And they're free to modify the prompt
templates as they see fit.

## Context of the tool

The Reforge tool is used by a single developer on their workstation, typically
at the start of a new software project. But they could also use it in an
existing project if they want.

The prompt templates copied to the project directory are committed to source
control by the developer so other developers can use the same templates. This
means that you typically use Reforge only once to get the prompt templates
installed in your project and after that sporadically to upgrade templates.
