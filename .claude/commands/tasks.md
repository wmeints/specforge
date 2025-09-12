# Creating tasks for the agent to implement

Create tasks Claude Code can implement in the project for $ARGUMENTS/PLAN.md. 
Follow the steps below.

## Steps

1. Read the `$ARGUMENTS/STORY.md` file to understand the user story.
2. Read the `$ARGUMENTS/RESEARCH.md` file to understand the relevant research.
3. Read the `$ARGUMENTS/PLAN.md` file to understand the implementation plan.
4. Create tasks for the story based on the information you gathered.
    - Order the tasks based on the phasing and dependencies.
    - Save the tasks in separate files to the `$ARGUMENTS/tasks` directory
    - Use the following naming pattern for the tasks `001-task-name.md`. 
5. Report the task files back to the user so they can verify that all tasks are there.

## Task guidelines

- Give each task a clear and short title.
- Give each task a clear description of what needs to be done.
- Make sure to include the full filenames of `STORY.md`, `RESEARCH.md`, and 
  `PLAN.md` files in the task file you create. This is important so Claude Code
  knows where to find key information for the task.




