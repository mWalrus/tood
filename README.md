# Tood: a simple TUI todo application

![Demo](./media/demo.gif)

## Install
### Pre-built binaries
Head [here](https://github.com/mWalrus/tood/releases) to grab a pre-built binary for your system.

### Manual steps
1. `git clone https://github.com/mWalrus/tood.git`
2. `cd tood`
3. `make all`
4. `tood`.

## Features

### Configurable keybinds
Every keybind in this application is configurable in [RON](https://github.com/ron-rs/ron).
Create the file `$HOME/.config/tood/key-config.ron` and add your keybinds there.

Change movement keys to VIM-like bindings:
```ron
(
  move_up: Some(( code: Char('k'), modifiers: ( bits: 0,),)),
  move_down: Some(( code: Char('j'), modifiers: ( bits: 0,),)),
  move_left: Some(( code: Char('h'), modifiers: ( bits: 0,),)),
  move_right: Some(( code: Char('l'), modifiers: ( bits: 0,),)),
  // sets scrolling to ctrl+u/ctrl+d
  desc_scroll_up: Some(( code: Char('u'), modifiers: ( bits: 2),)),
  desc_scroll_down: Some(( code: Char('d'), modifiers: ( bits: 2),)),
)
```
All modifiable keybinds can be found [here](https://github.com/mWalrus/tood/blob/main/src/keys/key_config.rs#L7).

### Configurable theme
You can configure the theme of the application to whatever you'd like
using the same method as for the keybinds above.

Tood has a default theme it loads unless you modify `~/.config/tood/theme.ron` like so:
```ron
// you can use an already defined color
(
  scrollbar: Some(Magenta)
)
```

```ron
// or you can define your own with rgb
(
  scrollbar: Some(Rgb(161, 38, 255))
)
```
All modifiable theme options can be found [here](https://github.com/mWalrus/tood/blob/main/src/theme/theme.rs#L11).
