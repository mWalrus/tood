# Features

## General
- [x] Move component rendering into each component
  - Example: instead of `views::todo_list` do `app.todos.render()`

## Navigation
- [x] Keyboard driven navigation (VIM binds currently)
- [x] Context based keybind hint bar (bottom of screen)
- [ ] Configurable keybinds

## Todo Manipulation
- [x] Create
- [x] Toggle completed
- [x] Edit
  - [x] Edit todo descriptions in $EDITOR
- [x] Delete
- [ ] Metadata
  - [x] Date and time added
  - [x] Date and time edited (empty if never edited)
  - [ ] Due date
    - [ ] Inline time/date highlighting (e.g. "do something __at 1:30 pm tomorrow__")
      - [ ] Parse due date from matches
- [x] Recurring todos
  - Not sure if we want more functionality surrounding this.
- [ ] Reorganize todos (move places)
  - Note: I'm thinking that this could be done by selecting the todo and pressing `m` for "move"
    which brings up a modal that lets the user input a number (1 = index 0 and so on). If the input
    isnt a number or is out of bounds in regards to the todo list then we return an error message back
    to app which will be displayed as a flash notification to the user.

## UI
- [x] Clear coloring
- [x] Flash messages communicating action outcomes
- [ ] Hint bar height adjustment when width is too small to fit it
- [ ] Style component holding the coloring for different components

## Fuzzy finder
- [x] Fuzzy todo finder
  - [x] Select correct todo when several todos have matching names
