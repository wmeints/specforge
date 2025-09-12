# Create an implementation plan for a story

Create an implementation plan for the story $ARGUMENTS
Follow the steps to write the plan.

## Steps

1. Read the story from $ARGUMENTS/STORY.md
2. Read the related feature specification. 
    - Find the feature in a directory under `docs/specifications/`. 
    - Read the `SPECIFICATION.md` file to understand how the story relates to the feature.
3. Research how to implement the user story described in $ARGUMENTS/STORY.md following the research guidelines.
4. Fill in the template `.templates/plan.md` following the implementation plan guidelines.
    - Store the plan in `PLAN.md` in the directory where the user story is described.
    - Run the review checklist that's part of the template.
5. Report the full path of the plan to the user so they know where it is located.

## Implementation plan guidelines

- Read `RESEARCH.md` in the user story directory to understand how the story can be implemented.
- Fill in all required sections of the implementation plan template.
- Define a solution you want to implement in code for each of the acceptance criteria.
- Define an automated test case for each of the test tasks in the user story.

## Research guidelines

Follow these steps to research the necessary elements in the implementation plan.

1. Define research tasks for each of the following elements
    - Define research tasks for elements in the technical context
    - Define research tasks for each of the implementation and test tasks
    - Define research tasks for each of the assumptions
    - Research relevant third-party libraries on the internet
    - Research best practices for libraries or modules the story depends on
2. Execute all the research tasks and consolidate the results in `RESEARCH.md` 
   in the user story directory.

## Asking questions about the implementation plan

1. Mark all ambiguous with [QUESTION: your question] for anything that is
   unclear or when you need to make an assumption on behalf of the developer.
2. Approach this as a software tester. Anything that's vague warrants a
   question as you can't test it.
