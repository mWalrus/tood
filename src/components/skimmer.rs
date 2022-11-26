use anyhow::Result;
use crossterm::event::{Event, KeyEvent};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use kanal::Sender;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use tui::Frame;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

use crate::app::{AppMessage, State};
use crate::keys::key_match;
use crate::keys::keymap::SharedKeyList;

use super::todo_list::Todo;
use super::Component;
use super::{utils, HIGHLIGHT_SYMBOL};

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
    pub state: ListState,
    pub input: Input,
    pub matches: Vec<SkimMatch>,
    keys: SharedKeyList,
    message_tx: Sender<AppMessage>,
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
    pub fn new(keys: SharedKeyList, message_tx: Sender<AppMessage>) -> Self {
        Self {
            state: ListState::default(),
            input: Input::default(),
            matches: Vec::new(),
            keys,
            message_tx,
        }
    }

    pub fn clear(&mut self) {
        self.state = ListState::default();
        self.matches.clear();
        self.input.reset();
    }

    pub fn skim(&mut self, todos: &[Todo]) {
        self.matches.clear();
        let matcher = Box::new(SkimMatcherV2::default());
        for (i, todo) in todos.iter().enumerate() {
            if let Some((score, indices)) = matcher.fuzzy_indices(&todo.name, self.input.value()) {
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

        if self.matches.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.matches.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.matches.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_match(&self) -> Option<&SkimMatch> {
        if let Some(i) = self.state.selected() {
            Some(&self.matches[i])
        } else {
            None
        }
    }
}

impl Component for SkimmerComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, _dim: bool) {
        let rect = utils::centered_rect(f.size());

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
            .highlight_style(
                Style::default()
                    .bg(Color::Indexed(8))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(HIGHLIGHT_SYMBOL);
        f.render_widget(Clear, chunks[0]);
        f.render_widget(Clear, chunks[1]);

        f.render_widget(skimmer_input, chunks[0]);
        f.render_stateful_widget(items, chunks[1], &mut self.state);
        f.set_cursor(
            chunks[0].x + (self.input.cursor() as u16).min(width) + 1,
            chunks[0].y + 1,
        );
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        if key_match(&key, &self.keys.back) {
            self.message_tx
                .send(AppMessage::InputState(State::Normal))?;
        } else if key_match(&key, &self.keys.alt_move_up) {
            self.previous();
        } else if key_match(&key, &self.keys.alt_move_down) {
            self.next();
        } else if key_match(&key, &self.keys.submit) {
            if let Some(s) = self.selected_match() {
                self.message_tx
                    .send(AppMessage::Skimmer(SkimmerAction::ReportSelection(
                        s.position,
                    )))?;
            }
            self.clear();
        } else {
            input_backend::to_input_request(Event::Key(key)).and_then(|r| self.input.handle(r));
            self.message_tx
                .send(AppMessage::Skimmer(SkimmerAction::Skim))?;
        }
        Ok(())
    }
}
