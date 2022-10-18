use crate::app::{App, PollOutcome, State};
use crate::components::{Component, MainComponent, StaticComponent};
use crate::widgets::hint_bar::HintBar;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};

pub fn run(mut app: App) -> TerminalResult<()> {
    let mut terminal = init_terminal().unwrap();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Err(e) = app.poll_event() {
            eprintln!("Failed to poll for key events: {e}");
            // idk if we want to break here or if we can recover.
            break;
        }

        match app.poll_message() {
            Ok(outcome) => match outcome {
                PollOutcome::NoAction => {}
                PollOutcome::ReInitTerminal => {
                    terminal = init_terminal()?;
                }
                PollOutcome::Break => break,
            },
            Err(e) => {
                restore_terminal()?;
                eprintln!("Failed to poll for app messages: {e}");
                break;
            }
        }
    }
    restore_terminal().unwrap();
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.state {
        State::Normal | State::Move => {
            app.todo_list.draw(f, false);
        }
        State::AddTodo | State::EditTodo => {
            app.todo_list.draw(f, true);
            app.todo_input.draw(f);
        }
        State::Find => {
            app.todo_list.draw(f, true);
            app.skimmer.draw(f);
        }
        State::DueDate => {
            app.todo_list.draw(f, true);
            app.due_date.draw(f);
        }
    }
    // draws notification if it exists
    app.notification.draw(f);
}

type TerminalResult<T> = std::result::Result<T, Box<dyn Error>>;

// Inits the terminal.
pub fn init_terminal() -> TerminalResult<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

// Resets the terminal.
pub fn restore_terminal() -> TerminalResult<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
