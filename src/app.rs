use super::components::SkimmerComponent;
use super::components::TodoListComponent;
use super::components::{notification::FlashMsg, NotificationComponent};
use crate::components::due_date::DueDateComponent;
use crate::components::skimmer::SkimmerAction;
use crate::components::todo_list::ListAction;
use crate::components::TodoInputComponent;
use crate::components::{Component, MainComponent};
use crate::keys::keymap::SharedKeyList;
use crate::widgets::hint_bar::BarType;
use crate::EVENT_TIMEOUT;
use anyhow::Result;
use crossterm::event;
use crossterm::event::Event;
use kanal::unbounded;
use kanal::Receiver;

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
    Flash(FlashMsg),
    RestoreTerminal,
    Quit,
}

pub enum PollOutcome {
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
    Move,
}

impl App {
    pub fn new(keys: SharedKeyList) -> App {
        let (sender, receiver) = unbounded::<AppMessage>();
        App {
            todo_list: TodoListComponent::load(keys.clone(), sender.clone()),
            todo_input: TodoInputComponent::new(keys.clone(), sender.clone()),
            skimmer: SkimmerComponent::new(keys.clone(), sender.clone()),
            notification: NotificationComponent::new(),
            due_date: DueDateComponent::new(keys.clone(), sender),
            keys,
            state: State::Normal,
            receiver,
        }
    }

    fn poll() -> Result<Option<Event>> {
        if event::poll(EVENT_TIMEOUT)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn poll_event(&mut self) -> Result<()> {
        if let Some(Event::Key(ev)) = Self::poll()? {
            match self.state {
                State::Normal | State::Move => {
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
                    self.due_date.handle_input(ev)?;
                }
            }
        }
        Ok(())
    }

    pub fn poll_message(&mut self) -> Result<PollOutcome> {
        if let Ok(Some(message)) = self.receiver.try_recv() {
            match message {
                AppMessage::InputState(state) => {
                    match state {
                        State::Find => {
                            self.todo_list.load_hintbar(BarType::Find);
                            let todos = self.todo_list.todos_ref();
                            self.skimmer.skim(todos);
                        }
                        State::EditTodo => {
                            self.todo_list.load_hintbar(BarType::Edit);
                            if let Some((t, i)) = self.todo_list.selected() {
                                self.todo_input.populate_with(t, i);
                            }
                        }
                        State::AddTodo => {
                            self.todo_list.load_hintbar(BarType::Edit);
                        }
                        State::Normal => {
                            self.todo_list.load_hintbar(BarType::Normal);
                        }
                        State::Move => {
                            self.todo_list.load_hintbar(BarType::Move);
                        }
                        State::DueDate => {
                            self.todo_list.load_hintbar(BarType::DueDate);
                        }
                    }
                    self.state = state;
                }
                AppMessage::Skimmer(skim_action) => match skim_action {
                    SkimmerAction::ReportSelection(s) => {
                        self.todo_list.select(s);
                        self.todo_list.load_hintbar(BarType::Normal);
                        // NOTE: I'm not sure if I like how we set the state without the sender.
                        //       It's not wrong but it's not really elegant imo.
                        self.state = State::Normal;
                        self.notification
                            .flash(FlashMsg::info("Entered normal mode"));
                    }
                    SkimmerAction::Skim => {
                        let todos = self.todo_list.todos_ref();
                        self.skimmer.skim(todos);
                    }
                },
                AppMessage::UpdateList(list_action) => {
                    match list_action {
                        ListAction::Add(t) => {
                            self.todo_list.add_todo(t)?;
                            self.notification.flash(FlashMsg::info("Added todo"));
                        }
                        ListAction::Replace(t, i) => {
                            self.todo_list.replace(t, i)?;
                            self.notification.flash(FlashMsg::info("Edited todo"));
                        }
                    }
                    self.todo_input.clear();
                    self.state = State::Normal;
                }
                AppMessage::Flash(flash_message) => self.notification.flash(flash_message),
                AppMessage::RestoreTerminal => return Ok(PollOutcome::ReInitTerminal),
                AppMessage::Quit => return Ok(PollOutcome::Break),
            }
        }

        Ok(PollOutcome::NoAction)
    }
}
