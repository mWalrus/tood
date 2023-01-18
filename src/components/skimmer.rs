use std::error::Error;

use anyhow::Result;
use crossterm::event::{Event, KeyEvent};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};
use tui::Frame;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;
use tui_utils::component::Component;
use tui_utils::keys::key_match;
use tui_utils::rect::centered_rect;
use tui_utils::state::BoundedState;
use tui_utils::LIST_HIGHLIGHT_SYMBOL;

use crate::app::{AppMessage, AppState};
use crate::keys::keymap::SharedKeyList;

use super::todo_list::Todo;
use super::utils;

pub enum SkimmerAction {
    Skim,
    ReportSelection(usize),
}

pub struct SkimMatch {
    pub text: String,
    pub position: usize,
    pub indices: Vec<usize>,
    score: i64,
}

pub struct SkimmerComponent {
    pub state: BoundedState,
    pub input: Input,
    pub matches: Vec<SkimMatch>,
    keys: SharedKeyList,
    matcher: Box<SkimMatcherV2>,
}

impl From<(usize, &Todo)> for SkimMatch {
    fn from(other: (usize, &Todo)) -> Self {
        Self {
            text: other.1.name.to_string(),
            position: other.0,
            indices: Vec::new(),
            score: 0,
        }
    }
}

impl SkimmerComponent {
    pub fn new(keys: SharedKeyList) -> Self {
        Self {
            state: BoundedState::default(),
            input: Input::default(),
            matches: Vec::new(),
            keys,
            matcher: Box::<SkimMatcherV2>::default(),
        }
    }

    pub fn clear(&mut self) {
        self.state = BoundedState::default();
        self.matches.clear();
        self.input.reset();
    }

    pub fn skim(&mut self, todos: &[Todo]) {
        self.matches.clear();
        for (i, todo) in todos.iter().enumerate() {
            if let Some((score, indices)) =
                self.matcher.fuzzy_indices(&todo.name, self.input.value())
            {
                let m = SkimMatch {
                    text: todo.name.clone(),
                    position: i,
                    indices,
                    score,
                };
                self.matches.push(m);
            }
        }
        self.matches.sort_by(|a, b| a.score.cmp(&b.score));
        self.state.update_boundary_from_vec(&self.matches);

        if self.matches.is_empty() {
            self.state.deselect();
        } else {
            self.state.select(0).unwrap();
        }
    }

    pub fn next(&mut self) {
        self.state.next();
    }

    pub fn previous(&mut self) {
        self.state.prev();
    }

    pub fn selected_match(&self) -> Option<&SkimMatch> {
        if let Some(i) = self.state.inner().selected() {
            Some(&self.matches[i])
        } else {
            None
        }
    }
}

impl Component for SkimmerComponent {
    type Message = AppMessage;

    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, _dim: bool) {
        let rect = centered_rect(f.size());

        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Max(10)].as_ref())
            .split(rect);

        let skimmer_input = &self.input;
        let width = chunks[0].width.max(3) - 3;
        let scroll = (skimmer_input.cursor() as u16).max(width) - width;
        let skimmer_input = Paragraph::new(skimmer_input.value())
            .scroll((0, scroll))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .title("Name"),
            );

        let width = chunks[1].width.max(3) - 3;

        let list_items: Vec<ListItem> = self
            .matches
            .iter()
            .map(|m| {
                let mut spans: Vec<Span> = Vec::with_capacity(m.text.len());
                for (i, c) in m.text.chars().enumerate() {
                    if m.indices.contains(&i) {
                        spans.push(Span::styled(
                            c.to_string(),
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::BOLD),
                        ));
                    } else {
                        spans.push(Span::raw(c.to_string()));
                    }
                }
                let spans = Spans::from(spans);
                ListItem::new(spans).style(Style::default())
            })
            .collect();

        let items = List::new(list_items)
            .block(utils::default_block("Todos"))
            .highlight_style(tui_utils::style::highlight_style())
            .highlight_symbol(LIST_HIGHLIGHT_SYMBOL);
        f.render_widget(Clear, chunks[0]);
        f.render_widget(Clear, chunks[1]);

        f.render_widget(skimmer_input, chunks[0]);
        f.render_stateful_widget(items, chunks[1], self.state.inner_mut());
        f.set_cursor(
            chunks[0].x + (self.input.cursor() as u16).min(width) + 1,
            chunks[0].y + 1,
        );
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<AppMessage, Box<dyn Error>> {
        if key_match(&key, &self.keys.back) {
            return Ok(AppMessage::InputState(AppState::Normal));
        } else if key_match(&key, &self.keys.alt_move_up) {
            self.previous();
        } else if key_match(&key, &self.keys.alt_move_down) {
            self.next();
        } else if key_match(&key, &self.keys.submit) {
            if let Some(s) = self.selected_match() {
                return Ok(AppMessage::Skimmer(SkimmerAction::ReportSelection(
                    s.position,
                )));
            }
            self.clear();
        } else {
            input_backend::to_input_request(Event::Key(key)).and_then(|r| self.input.handle(r));
            return Ok(AppMessage::Skimmer(SkimmerAction::Skim));
        }
        Ok(AppMessage::NoAction)
    }
}
