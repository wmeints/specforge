# Creating a feature specification

Start a new feature by creating a specification and feature branch.

## Steps

1. Read the user input and choose a name for the new feature.
2. Use the git command-line utility to create a new branch with the name of the feature.
   Use `feature/<feature-name>` for the feature branch.
3. Read the template `.github/ISSUE_TEMPLATE/feature-specification.md` to understand how
   to build the specification.
4. Write the specification to a markdown file in
   `docs/specifications/001-feature-name/feature.md`. Use the structure described in the
   template you read. Make sure to correctly name the file following the file naming
   guidelines below. Give the feature a unique number.
5. Report back to the user with the name of the branch and the filename of the
   specification so they can iterate on the specification.

## Workflow for filling in the feature specification template

1. Read the user description from the input.
2. Extract the key concepts from the user's description.
3. Fill in the user scenarios and testing section
4. Define functional requirements for the feature
5. Identify entities involved in the feature (if data is involved)

## Important guidelines for writing a feature specification

- Focus on the WHAT and WHY of the feature
- Avoid discussing HOW the feature should be implemented for now.
- Write the feature description for business takeholders.

## Asking questions about the feature

1. Mark all ambiguous with [QUESTION: your question] for anything that is
   unclear or when you need to make an assumption on behalf of the developer.
2. Use the INVEST approach when filling in sections. The developer wants
   feature specifications with clear boundaries and value.
3. Approach this as a software tester. Anything that's vague warrants a
   question as you can't test it.

## Feature Specification Section guidelines

- Make sure the required sections are filled
- Include optional sections as necessary
- When a section doesn't apply, remove it completely

## Important background information

- The information in `docs/architecture/01-introduction.md` is important
  as background information on what the application is supposed to do.
- Consider the constraints documented in `docs/architecture/02-constraints.md`
  when writing the feature specification.
