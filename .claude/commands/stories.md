# Planning stories for a feature

Create user stories for the feature specified in `$ARGUMENTS/SPECIFICATION.md`.
Follow the steps below to effectively create user stories.

## Steps

1. Read the specification from the `$ARGUMENTS/SPECIFICATION.md` file.
2. Define the user stories following the story definition guidelines.
3. Order the user stories by their dependencies.
    - Stories needed to set up a feature should go first.
    - Stories defining the most basic version of the feature should go next.
    - Stories refining the feature with extra alternative scenarios should go after that.
    - Finally, stories with edge cases should go last.
4. Number the stories in order, and write them to disk.
    - Create a directory per story following the format `$ARGUMENTS/stories/001-story-title`.
    - Write the story file to `STORY.md` in the story directory.
    - Fill in the template from `.templates/story.md` for the content of the story file.
    - Run the story review checklist that is part of the story template for each story.
5. Report the story filenames to the user so they know where to find the stories.

## Story definition guidelines

- Base stories of the information found in the feature specification.
- Create a story for the primary scenario in the feature.
- Create a story for each acceptance scenario in the feature specification.
- Create a separate story for each of the edge cases.
- Create a separate story for setting up data structures for the feature.

## Story template guidelines

- Fill in the required sections.
- Include information for optional sections when necessary.
- Focus on the WHAT and WHY of the feature.
- Avoid discussing HOW the feature should be implemented for now.
- Write the story for business stakeholders.

## Asking questions about stories

1. Mark all ambiguous with [QUESTION: your question] for anything that is
   unclear or when you need to make an assumption on behalf of the developer.
2. Use the INVEST approach when filling in sections of the stories. The
   developer wants feature specifications with clear boundaries and value.
3. Approach this as a software tester. Anything that's vague warrants a
   question as you can't test it.