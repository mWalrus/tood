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
    pub calendar_state: ListState,
    pub calendar: Calendar,
}

impl Component for TodoInput {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let rect = utils::centered_rect(f.size());

        if self.calendar.is_visible {
            f.render_widget(Clear, rect);
            // FIXME: avoid clone
            f.render_stateful_widget(self.calendar.clone(), rect, &mut self.calendar_state);
            return;
        }

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
