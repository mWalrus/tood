use crossterm::event::{Event, KeyEvent};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use tui::widgets::ListState;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

use super::todo_list::Todo;

pub struct SkimMatch {
    pub text: String,
    pub indices: Vec<usize>,
    score: i64,
}

#[derive(Default)]
pub struct Skimmer {
    pub state: ListState,
    pub input: Input,
    pub matches: Vec<SkimMatch>,
}

impl From<&Todo> for SkimMatch {
    fn from(other: &Todo) -> Self {
        Self {
            text: other.name.to_string(),
            indices: Vec::new(),
            score: 0,
        }
    }
}

impl Skimmer {
    pub fn skim(&mut self, ev: Option<KeyEvent>, todos: &[Todo]) {
        if let Some(e) = ev {
            input_backend::to_input_request(Event::Key(e)).and_then(|r| self.input.handle(r));
            let mut matches: Vec<SkimMatch> = Vec::new();
            let matcher = Box::new(SkimMatcherV2::default());
            for todo in todos {
                if let Some((score, indices)) =
                    matcher.fuzzy_indices(&todo.name, &self.input.value())
                {
                    let m = SkimMatch {
                        text: todo.name.clone(),
                        indices,
                        score,
                    };
                    matches.push(m);
                }
            }
            matches.sort_by(|a, b| a.score.cmp(&b.score));
            self.matches = matches;
        } else {
            self.matches = todos.iter().map(|t| SkimMatch::from(t)).collect();
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
}
