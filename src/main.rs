mod app;
mod components;
mod keys;
mod message;
mod ui;
mod widgets;

use app::App;
use keys::{keymap::SharedKeyList, ToodKeyList};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let keys = SharedKeyList::new(ToodKeyList::init());
    let app = App::new(keys.clone());

    let res = ui::run(app);

    if let Err(e) = res {
        println!("{e:#?}");
    }

    Ok(())
}
