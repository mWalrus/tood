use super::components::SkimmerComponent;
use super::components::TodoListComponent;
use super::components::{notification::NotificationMessage, NotificationComponent};
use crate::components::due_date::DueDateComponent;
use crate::components::skimmer::SkimmerAction;
use crate::components::todo_list::ListAction;
use crate::components::TodoInputComponent;
use crate::components::{Component, MainComponent};
use crate::keys::keymap::SharedKeyList;
use anyhow::Result;
use crossterm::event;
use crossterm::event::Event;
use kanal::unbounded;
use kanal::Receiver;
use std::time::Duration;

static POLL_DURATION: Duration = Duration::from_millis(1000);

pub struct App {
    pub todo_list: TodoListComponent,
    pub todo_input: TodoInputComponent,
    pub skimmer: SkimmerComponent,
    pub notification: NotificationComponent,
    pub due_date: DueDateComponent,
    pub keys: SharedKeyList,
    pub state: State,
    receiver: Receiver<AppMessage>,
}

pub enum AppMessage {
    InputState(State),
    Skimmer(SkimmerAction),
    UpdateList(ListAction),
    RestoreTerminal,
    Quit,
}

pub enum PollResponse {
    NoAction,
    ReInitTerminal,
    Break,
}

#[derive(Eq, PartialEq)]
pub enum State {
    Normal,
    AddTodo,
    EditTodo,
    DueDate,
    Find,
}

impl App {
    pub fn new(keys: SharedKeyList) -> App {
        let (sender, receiver) = unbounded::<AppMessage>();
        App {
            todo_list: TodoListComponent::load(keys.clone(), sender.clone()),
            todo_input: TodoInputComponent::new(keys.clone(), sender.clone()),
            skimmer: SkimmerComponent::new(keys.clone(), sender.clone()),
            notification: NotificationComponent::new(),
            due_date: DueDateComponent::new(keys.clone()),
            keys: keys.clone(),
            state: State::Normal,
            receiver,
        }
    }

    fn poll(duration: Duration) -> Result<Option<Event>> {
        if event::poll(duration)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn poll_event(&mut self) -> Result<PollResponse> {
        if let Some(Event::Key(ev)) = Self::poll(POLL_DURATION)? {
            match self.state {
                State::Normal => {
                    self.todo_list.handle_input(ev)?;
                }
                State::AddTodo => {
                    self.todo_input.handle_input(ev)?;
                }
                State::EditTodo => {
                    self.todo_input.handle_input(ev)?;
                }
                State::Find => {
                    self.skimmer.handle_input(ev)?;
                }
                State::DueDate => {
                    todo!()
                }
            }
        }

        if let Ok(Some(message)) = self.receiver.try_recv() {
            match message {
                AppMessage::InputState(state) => {
                    match state {
                        State::Find => {
                            self.notification
                                .set(NotificationMessage::info("Entered find mode"));
                            let todos = self.todo_list.todos_ref();
                            self.skimmer.skim(&todos);
                        }
                        State::EditTodo => {
                            self.notification
                                .set(NotificationMessage::info("Entered edit mode"));
                            if let Some((t, i)) = self.todo_list.selected() {
                                self.todo_input.populate_with(t, i);
                            }
                        }
                        State::AddTodo => {
                            self.notification
                                .set(NotificationMessage::info("Entered edit mode"));
                        }
                        State::Normal => {
                            self.notification
                                .set(NotificationMessage::info("Entered normal mode"));
                        }
                        _ => {}
                    }
                    self.state = state;
                }
                AppMessage::Skimmer(skim_action) => match skim_action {
                    SkimmerAction::ReportSelection(s) => {
                        self.todo_list.select(s);
                        // NOTE: I'm not sure if I like how we set the state without the sender.
                        //       It's not wrong but it's not really elegant imo.
                        self.state = State::Normal;
                        self.notification
                            .set(NotificationMessage::info("Entered normal mode"));
                    }
                    SkimmerAction::Skim => {
                        let todos = self.todo_list.todos_ref();
                        self.skimmer.skim(&todos);
                    }
                },
                AppMessage::UpdateList(list_action) => {
                    // placeholder
                    match list_action {
                        ListAction::Add(t) => self.todo_list.add_todo(t),
                        ListAction::Replace(t, i) => self.todo_list.replace(t, i),
                    }
                    self.todo_input.clear();
                    self.state = State::Normal;
                    self.notification
                        .set(NotificationMessage::info("Entered normal mode"));
                }
                AppMessage::RestoreTerminal => return Ok(PollResponse::ReInitTerminal),
                AppMessage::Quit => return Ok(PollResponse::Break),
            }
        }

        // clear the current flashed notification from the screen
        if self.notification.rx.try_recv().is_ok() {
            self.notification.clear();
        }

        Ok(PollResponse::NoAction)
    }
}
