use crate::components::app::{App, InputMode};
use crate::components::hint_bar::HintBar;
use crate::components::{Component, MainComponent};
use crate::keys::key_match;
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
                        app.enter_mode(InputMode::Edit);
                    } else if key_match(&key, &app.keys.find_mode) {
                        app.enter_mode(InputMode::Find);
                    } else if key_match(&key, &app.keys.move_mode) {
                        app.enter_mode(InputMode::Move);
                    } else if key_match(&key, &app.keys.edit_todo) {
                        app.edit_todo();
                    } else if key_match(&key, &app.keys.toggle_completed) {
                        app.toggle_todo_completed();
                    } else if key_match(&key, &app.keys.remove_todo) {
                        app.remove_current_todo();
                    }
                }
                InputMode::Edit => {
                    if key_match(&key, &app.keys.back) {
                        app.enter_mode(InputMode::Normal);
                    } else if key_match(&key, &app.keys.submit) {
                        app.add_todo();
                    } else if key_match(&key, &app.keys.add_description) {
                        app.edit_description();
                        terminal = init_terminal().unwrap();
                    } else if key_match(&key, &app.keys.mark_recurring) {
                        app.toggle_recurring();
                    } else {
                        app.todos.handle_input(key);
                    }
                }
                InputMode::Find => {
                    if key_match(&key, &app.keys.back) {
                        app.enter_mode(InputMode::Normal);
                    } else if key_match(&key, &app.keys.alt_move_up) {
                        app.skimmer.previous();
                    } else if key_match(&key, &app.keys.alt_move_down) {
                        app.skimmer.next();
                    } else if key_match(&key, &app.keys.submit) {
                        app.load_fuzzy_selection();
                        app.mode = InputMode::Normal;
                    } else {
                        app.skimmer.skim(Some(key), &app.todos.todos);
                    }
                }
                InputMode::Move => {
                    if key_match(&key, &app.keys.submit) {
                        app.enter_mode(InputMode::Normal);
                    } else if key_match(&key, &app.keys.move_up) {
                        app.todos.move_todo_up();
                    } else if key_match(&key, &app.keys.move_down) {
                        app.todos.move_todo_down();
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    match app.mode {
        InputMode::Normal => {
            app.todos.draw(f, false, HintBar::normal_mode(app));
        }
        InputMode::Edit => {
            app.todos.draw(f, true, HintBar::edit_mode(app));
            app.todos.new_todo.draw(f);
        }
        InputMode::Find => {
            app.todos.draw(f, true, HintBar::find_mode(app));
            app.skimmer.draw(f);
        }
        InputMode::Move => {
            app.todos.draw(f, false, HintBar::move_mode(app));
        }
    }
    // draws notification if it exists
    app.notification.draw(f);
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
