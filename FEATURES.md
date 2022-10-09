# Features

## General

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
  - [ ] Date and time added
  - [ ] Due date
    - [ ] Inline time/date highlighting (e.g. "do something __at 1:30 pm tomorrow__")
      - [ ] Parse due date from matches
- [ ] Recurring todos

## UI
- [x] Clear coloring
- [x] Flash messages communicating action outcomes

## Fuzzy finder
- [x] Fuzzy todo finder
  - Note: Currently we don't handle different todos with the same name.
          This means that if you fuzzy search for a todo named "testing"
          and there is another todo called "testing",  the fuzzy matcher
          will return the index for the first one in the list no matter
          if you chose the second one.
