# Tood: a simple TUI todo application

![Demo](./media/demo.gif)

## Install
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
)
```

All modifiable keybinds can be found [here](https://github.com/mWalrus/tood/blob/main/src/keys/key_config.rs#L7).