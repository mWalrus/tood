mod app;
mod components;
mod keys;
mod ui;
mod widgets;

use app::App;
use keys::{keymap::SharedKeyList, ToodKeyList};
use std::{error::Error, time::Duration};

pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);

fn main() -> Result<(), Box<dyn Error>> {
    let keys = SharedKeyList::new(ToodKeyList::init());
    let app = App::new(keys.clone());

    let res = ui::run(app);

    if let Err(e) = res {
        println!("{e:#?}");
    }

    Ok(())
}
