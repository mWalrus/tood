use crate::app::{App, AppMessage, AppState};
use crossterm::event::Event;
use ratatui::{backend::Backend, Frame};
use std::error::Error;
use tui_utils::{component::Component, term};

pub fn run(mut app: App) -> TerminalResult<()> {
    let mut terminal = term::init().unwrap();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // then handle input events
        let event_outcome = match term::poll_event() {
            Ok(Some(Event::Key(ev))) => match app.state {
                AppState::Normal | AppState::Move => app.todo_list.handle_input(ev),
                AppState::AddTodo => app.todo_input.handle_input(ev),
                AppState::EditTodo => app.todo_input.handle_input(ev),
                AppState::Find => app.skimmer.handle_input(ev),
                AppState::DueDate => app.due_date.handle_input(ev),
            },
            // other term events, we dont handle them in this example
            Ok(Some(_)) => Ok(AppMessage::NoAction),
            // no events were found
            Ok(None) => Ok(AppMessage::NoAction),
            // something went wrong
            Err(e) => Err(e.into()),
        };

        // display any notifications in the queue
        app.poll_flash_messages();

        match event_outcome {
            Ok(AppMessage::NoAction) => {}
            Ok(AppMessage::InputState(state)) => app.update_state(state)?,
            Ok(AppMessage::Skimmer(skim_action)) => app.perform_skimmer_action(skim_action),
            Ok(AppMessage::UpdateList(list_action)) => app.todo_list_action(list_action)?,
            Ok(AppMessage::SetDueDate(d)) => app.set_due_date(d),
            Ok(AppMessage::ReInitTerminal) => terminal = term::init().unwrap(),
            Ok(AppMessage::Quit) => {
                term::restore().unwrap();
                break;
            }
            Err(e) => {
                term::restore_with_err(e).unwrap();
                break;
            }
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.state {
        AppState::Normal | AppState::Move => {
            app.todo_list.draw(f, false);
        }
        AppState::AddTodo | AppState::EditTodo => {
            app.todo_list.draw(f, true);
            app.todo_input.draw(f, false);
        }
        AppState::Find => {
            app.todo_list.draw(f, true);
            app.skimmer.draw(f, false);
        }
        AppState::DueDate => {
            app.todo_list.draw(f, true);
            app.due_date.draw(f, false);
        }
    }
    // draws notification if it exists
    app.notification.draw(f, false);
}

type TerminalResult<T> = std::result::Result<T, Box<dyn Error>>;
