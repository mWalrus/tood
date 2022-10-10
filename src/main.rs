mod components;
mod keymap;
mod ui;

use components::app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::default();

    let res = ui::run(app);

    if let Err(e) = res {
        println!("{e:#?}");
        ui::reset_terminal()?;
    }

    Ok(())
}
