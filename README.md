# Focus: Opinionated task management

This project is two main items:
1. An integration layer to sync between the multiple task management systems I use: 
    - Obsidian for collecting todos throughout the day
    - Jira for work tasks
    - TaskWarrior, where I actually manage my work queue and roadmap
2. A CLI for working with tasks in TaskWarrior the way I want (described under "Workflow" below)

# Integration

(WIP)

This project provides a "middle-end" for syncing from Obsidian to TaskWarrior, JIRA to TaskWarrior, and TaskWarrior to JIRA.
It contains a model of how I think of tasks and what attributes / properties I use to manage tasks, and code for syncing from
that model to the other systems. It does not store any tasks on its own -- instead it loads from one or more "backends" (Obsidian, 
TaskWarrior, or JIRA) into that canonical model, and can propagate diffs from one system to another.

I'm not trying to solve any generalized synchronization problems, so, e.g., conflicts are things this can't handle. If a task
has been updated in more than one system, we'll just identify that and bail out, leaving the conflicts to be resolved by handle.

# Workflow

This project implements a few operations for helping to enable my personal favorite task management strategy in
TaskWarrior. It's a two-layer strategy where I have some set of tasks that are my currently prioritized -- the ones
I'm focusing on; and everything else is in the backlog.

There are six operations available:
1. *Focus* on a task: move a task from the backlog to the focused set
2. *Unfocus* a task: move a task from the focused set to the backlog
3. *Complete* a task: mark the task as complete and remove it from the focused set or backlog
4. *Delete* a task: remove the task entirely from the focused set or backlog
5. *Prioritize* a task: move a task to the front of the focused set
6. *Deprioritize* a task: move a task to the end of the focused set

## Implementation in TaskWarrior

1. *Focus*: add the `+focus` tag to a task
2. *Unfocus*: remove the `+focus` tag from a task
3. *Complete* a task: `task [id] done`
4. *Delete* a task: `task [id] delete`
5. *Prioritize* a task: update the `sortOrder` UDA on a focused task to be less than the least sortOrder in the focused set. (Since we sort in ascending order by `sortOrder`, that moves it to the front of the focused set.)
6. *Depriorize* a task: update the `sortOrder` UDA on a focused task to be greater than the greatest sortOrder in the focused set.

This project is a frontend to TaskWarrior for performing those six operations, plus various housekeeping (like removing
`sortOrder` from tasks in the backlog and compacting the existing `sortOrder`s so they're not super spread out after
a bunch of `prioritize` and `deprioritize` operations have taken place.


