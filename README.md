# Tood: a simple CLI todo manager

![Demo](./media/tood-demo.gif)

## Install
1. `git clone https://gitlab.com/mWalrus/tood.git`
2. `cd tood`
3. `make all`
4. `tood`.

## Features

### Configurable keybinds
Every keybind in this application is configurable in [RON](https://github.com/ron-rs/ron).
Create the file `$HOME/.config/tood/key-config.ron` and add your keybinds there.

Example for using arrow up and down to move:
```ron
(
  move_up: Some(( code: Up, modifiers: ( bits: 0,),)),
  move_down: Some(( code: Down, modifiers: ( bits: 0,),)),
)
```

All modifiable keybinds can be found [here](https://github.com/mWalrus/tood/blob/main/src/keys/key_config.rs#L7).