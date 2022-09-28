mod input;
mod types;
mod ui;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs_next as dirs;
use lazy_static::lazy_static;
use std::{error::Error, io, path::PathBuf};
use tui::{backend::CrosstermBackend, Terminal};
use types::App;

lazy_static! {
    pub(crate) static ref TODO_FILE: PathBuf = {
        let todos = dirs::config_dir().unwrap().join("tood");
        std::fs::create_dir_all(&todos).unwrap();
        let todos = todos.join("todos.toml");
        std::fs::File::create(&todos).unwrap();
        todos
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = ui::run(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        println!("{e:#?}");
    }

    Ok(())
}
