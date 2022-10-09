pub mod hint_bar;
pub mod notification;
mod utils;
pub mod views;

use crate::keymap::key_match;
use crate::types::app::{App, InputMode};
use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};

pub fn run(mut app: App) -> io::Result<()> {
    let mut terminal = init_terminal().unwrap();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // clear the current flashed notification from the screen
        if app.notification.rx.try_recv().is_ok() {
            app.notification.clear();
        }

        if let Event::Key(key) = event::read()? {
            match app.mode {
                InputMode::Normal => {
                    if key_match(&key, &app.keys.quit) {
                        reset_terminal().unwrap();
                        return Ok(());
                    } else if key_match(&key, &app.keys.move_up) {
                        app.todos.previous();
                    } else if key_match(&key, &app.keys.move_down) {
                        app.todos.next();
                    } else if key_match(&key, &app.keys.add_todo) {
                        app.mode = InputMode::Editing;
                    } else if key_match(&key, &app.keys.find) {
                        app.mode = InputMode::Find;
                        app.skimmer.skim(None, &app.todos.todos);
                    } else if key_match(&key, &app.keys.edit_todo) {
                        app.edit_todo();
                    } else if key_match(&key, &app.keys.toggle_completed) {
                        app.toggle_todo_completed();
                    } else if key_match(&key, &app.keys.remove_todo) {
                        app.remove_current_todo();
                    }
                }
                InputMode::Editing => {
                    if key_match(&key, &app.keys.back) {
                        app.reset_state();
                    } else if key_match(&key, &app.keys.submit) {
                        app.add_todo();
                    } else if key_match(&key, &app.keys.add_description) {
                        reset_terminal().unwrap();
                        app.edit_description();
                        terminal = init_terminal().unwrap();
                    } else {
                        app.todos.handle_input(key);
                    }
                }
                InputMode::Find => {
                    if key_match(&key, &app.keys.back) {
                        app.reset_state();
                    } else if key_match(&key, &app.keys.secondary_move_up) {
                        app.skimmer.previous();
                    } else if key_match(&key, &app.keys.secondary_move_down) {
                        app.skimmer.next();
                    } else if key_match(&key, &app.keys.submit) {
                        app.load_fuzzy_selection();
                        app.mode = InputMode::Normal;
                    } else {
                        app.skimmer.skim(Some(key), &app.todos.todos);
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // NOTE: we currently render the main application view
    //       no matter the mode we're in, so lets keep this here for now
    views::todo_list(app, f);
    match app.mode {
        InputMode::Normal => {
            let binds = [
                ("Up", app.keys.move_up.to_string()),
                ("Down", app.keys.move_down.to_string()),
                ("Add", app.keys.add_todo.to_string()),
                ("Find", app.keys.find.to_string()),
                ("Toggle", app.keys.toggle_completed.to_string()),
                ("Edit", app.keys.edit_todo.to_string()),
                ("Delete", app.keys.remove_todo.to_string()),
                ("Quit", app.keys.quit.to_string()),
            ];
            hint_bar::draw(f, &binds);
        }
        InputMode::Editing => {
            views::edit_modal(app, f);
            let binds = [
                ("Back", app.keys.back.to_string()),
                ("Edit desc", app.keys.add_description.to_string()),
                ("Save", app.keys.submit.to_string()),
            ];
            hint_bar::draw(f, &binds);
        }
        InputMode::Find => {
            views::fuzzy_matcher(app, f);
            let binds = [
                ("Back", app.keys.back.to_string()),
                ("Up", app.keys.secondary_move_up.to_string()),
                ("Down", app.keys.secondary_move_down.to_string()),
                ("Select", app.keys.submit.to_string()),
            ];
            hint_bar::draw(f, &binds);
        }
    }
    // draws notification if it exists
    notification::draw(app, f);
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Inits the terminal.
pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

// Resets the terminal.
pub fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
