mod keymap;
mod types;
mod ui;

use std::error::Error;
use types::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::default();

    let res = ui::run(app);

    if let Err(e) = res {
        println!("{e:#?}");
        ui::reset_terminal()?;
    }

    Ok(())
}
