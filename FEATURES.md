# Features

## General
- [x] Move component rendering into each component
  - Example: instead of `views::todo_list` do `app.todos.render()`
- [x] RON configuration language

## Navigation
- [x] Keyboard driven navigation (VIM binds currently)
- [x] Context based keybind hint bar (bottom of screen)
- [x] Configurable keybinds
- [ ] Scroll through todo description

## Todo Manipulation
- [x] Create
- [x] Toggle completed
- [x] Edit
  - [x] Edit todo descriptions in $EDITOR
- [x] Delete
- [x] Metadata
  - [x] Date and time added
  - [x] Date and time edited (empty if never edited)
  - [x] Due date
- [x] Recurring todos
  - Not sure if we want more functionality surrounding this.
- [x] Reorganize todos (move places)

## UI
- [x] Clear coloring
- [x] Flash messages communicating action outcomes
- [x] Hint bar height adjustment when width is too small to fit it
- [ ] Style component holding the coloring for different components
- [x] Gray out borders of background view when rendering modals

## Fuzzy finder
- [x] Fuzzy todo finder
  - [x] Select correct todo when several todos have matching names
