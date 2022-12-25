use super::components::SkimmerComponent;
use super::components::TodoListComponent;
use super::components::{notification::FlashMsg, NotificationComponent};
use crate::components::due_date::DueDateComponent;
use crate::components::skimmer::SkimmerAction;
use crate::components::todo_list::ListAction;
use crate::components::TodoInputComponent;
use crate::keys::keymap::SharedKeyList;
use crate::widgets::hint_bar::BarType;
use anyhow::Result;
use chrono::NaiveDateTime;
use kanal::unbounded;
use kanal::Receiver;

pub struct App {
    pub todo_list: TodoListComponent,
    pub todo_input: TodoInputComponent,
    pub skimmer: SkimmerComponent,
    pub notification: NotificationComponent,
    pub due_date: DueDateComponent,
    pub keys: SharedKeyList,
    pub state: AppState,
    flash_rx: Receiver<FlashMsg>,
}

#[derive(Default)]
pub enum AppMessage {
    InputState(AppState),
    Skimmer(SkimmerAction),
    UpdateList(ListAction),
    SetDueDate(NaiveDateTime),
    ReInitTerminal,
    #[default]
    NoAction,
    Quit,
}

#[derive(Eq, PartialEq)]
pub enum AppState {
    Normal,
    AddTodo,
    EditTodo,
    DueDate,
    Find,
    Move,
}

impl App {
    pub fn new(keys: SharedKeyList) -> App {
        let (sender, receiver) = unbounded::<FlashMsg>();
        App {
            todo_list: TodoListComponent::load(keys.clone(), sender.clone()),
            todo_input: TodoInputComponent::new(keys.clone()),
            skimmer: SkimmerComponent::new(keys.clone()),
            notification: NotificationComponent::new(),
            due_date: DueDateComponent::new(keys.clone(), sender),
            keys,
            state: AppState::Normal,
            flash_rx: receiver,
        }
    }

    pub fn poll_flash_messages(&mut self) {
        if let Ok(Some(msg)) = self.flash_rx.try_recv() {
            self.notification.flash(msg);
        }
    }

    pub fn update_state(&mut self, state: AppState) -> Result<()> {
        match state {
            AppState::Find => {
                self.todo_list.load_hintbar(BarType::Find);
                let todos = self.todo_list.todos_ref();
                self.skimmer.skim(todos);
            }
            AppState::EditTodo => {
                self.todo_list.load_hintbar(BarType::Edit);
                if let Some((t, i)) = self.todo_list.selected() {
                    self.todo_input.populate_with(t, i);
                }
            }
            AppState::AddTodo => {
                self.todo_list.load_hintbar(BarType::Edit);
            }
            AppState::Normal => {
                self.todo_list.load_hintbar(BarType::Normal);
            }
            AppState::Move => {
                self.todo_list.load_hintbar(BarType::Move);
            }
            AppState::DueDate => {
                self.todo_list.load_hintbar(BarType::DueDate);
                if self.state == AppState::EditTodo {
                    if let Some(dt) = self.todo_input.get_due_date() {
                        self.due_date.set_date_time(dt)?;
                    }
                }
            }
        }
        self.state = state;
        Ok(())
    }

    pub fn perform_skimmer_action(&mut self, skimmer_action: SkimmerAction) {
        match skimmer_action {
            SkimmerAction::ReportSelection(s) => {
                self.todo_list.select(s);
                self.todo_list.load_hintbar(BarType::Normal);

                self.state = AppState::Normal;
                self.notification
                    .flash(FlashMsg::info("Entered normal mode"));
            }
            SkimmerAction::Skim => {
                let todos = self.todo_list.todos_ref();
                self.skimmer.skim(todos);
            }
        }
    }

    pub fn todo_list_action(&mut self, list_action: ListAction) -> Result<()> {
        let msg = match list_action {
            ListAction::Add(t) => {
                self.todo_list.add_todo(t)?;
                "Added todo"
            }
            ListAction::Replace(t, i) => {
                self.todo_list.replace(t, i)?;
                "Edited todo"
            }
        };

        self.notification.flash(FlashMsg::info(msg));
        self.todo_list.load_hintbar(BarType::Normal);
        self.todo_input.clear();
        self.state = AppState::Normal;
        Ok(())
    }

    pub fn set_due_date(&mut self, d: NaiveDateTime) {
        self.todo_input.set_due_date(d);
        self.state = AppState::AddTodo;
    }
}
