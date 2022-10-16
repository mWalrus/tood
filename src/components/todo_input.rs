use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, ListState, Paragraph},
    Frame,
};
use tui_input::Input;

use crate::widgets::calendar::Calendar;

use super::{utils, Component};

#[derive(Default, Debug, Clone)]
pub struct TodoInput {
    pub name: Input,
    pub description: String,
    pub recurring: bool,
    pub is_editing_existing: bool,
    // FIXME: implement From<NaiveDateTime> for ListState
    pub calendar_state: ListState,
    pub calendar: Calendar,
}

impl TodoInput {
    pub fn cal_right(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i >= self.calendar.cells.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }

    pub fn cal_left(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.calendar.cells.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }

    pub fn cal_down(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i + 7 >= self.calendar.cells.len() - 1 {
                    self.calendar.cells.len() - 1
                } else {
                    i + 7
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }

    pub fn cal_up(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i < 7 {
                    0
                } else {
                    i - 7
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }
}

impl Component for TodoInput {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.calendar.is_visible {
            let rect = utils::calendar_rect(f.size());
            f.render_widget(Clear, rect);
            // FIXME: avoid clone
            f.render_stateful_widget(self.calendar.clone(), rect, &mut self.calendar_state);
            return;
        }

        let rect = utils::centered_rect(f.size());

        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Max(10)].as_ref())
            .split(rect);

        let name_input = &self.name;
        let width = chunks[0].width.max(3) - 3;
        let scroll = (name_input.cursor() as u16).max(width) - width;
        let name_input = Paragraph::new(name_input.value())
            .scroll((0, scroll))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .title("Name"),
            );

        let width = chunks[1].width.max(3) - 3;
        let desc_input = Paragraph::new(&*self.description)
            .wrap(tui::widgets::Wrap { trim: true })
            .block(utils::default_block("Description"));

        f.render_widget(Clear, chunks[0]);
        f.render_widget(Clear, chunks[1]);

        f.render_widget(name_input, chunks[0]);
        f.render_widget(desc_input, chunks[1]);
        f.set_cursor(
            chunks[0].x + (self.name.cursor() as u16).min(width) + 1,
            chunks[0].y + 1,
        );
    }
}
