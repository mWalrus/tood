pub mod hint_bar;
pub mod notification;
pub mod views;

use crate::keymap::key_match;
use crate::types::app::{App, InputMode};
use crate::types::notification::ToodMsgType;
use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
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
                    } else if key_match(&key, &app.keys.save_new_todo) {
                        app.add_todo();
                    } else if key_match(&key, &app.keys.add_description) {
                        reset_terminal().unwrap();
                        app.edit_description();
                        terminal = init_terminal().unwrap();
                    } else {
                        app.handle_input_event(key);
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.mode {
        InputMode::Normal => {
            views::todo_list(app, f);

            let binds = [
                ("Up", app.keys.move_up.to_string()),
                ("Down", app.keys.move_down.to_string()),
                ("Add", app.keys.add_todo.to_string()),
                ("Toggle", app.keys.toggle_completed.to_string()),
                ("Edit", app.keys.edit_todo.to_string()),
                ("Delete", app.keys.remove_todo.to_string()),
                ("Quit", app.keys.quit.to_string()),
            ];
            hint_bar::draw(f, &binds);
        }
        InputMode::Editing => {
            views::todo_list(app, f);
            views::edit_modal(app, f);

            let binds = [
                ("Back", app.keys.back.to_string()),
                ("Add desc", app.keys.add_description.to_string()),
                ("Save", app.keys.save_new_todo.to_string()),
            ];
            hint_bar::draw(f, &binds);
        }
    }
    // draws notification if set
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