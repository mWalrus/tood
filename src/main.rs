mod app;
mod components;
#[macro_use]
mod config;
mod keys;
mod theme;
mod ui;
mod widgets;

use app::App;
use std::{error::Error, time::Duration};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let app = App::new();

    let res = ui::run(app);

    if let Err(e) = res {
        println!("{e:#?}");
    }

    Ok(())
}
