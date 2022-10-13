use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{app::App, Component};

pub struct HintBar {
    hints: Vec<Hint>,
}

pub struct Hint {
    name: &'static str,
    bind: String,
}

impl HintBar {
    pub fn normal_mode(app: &App) -> Self {
        let hints = vec![
            Hint {
                name: "Up",
                bind: app.keys.move_up.to_string(),
            },
            Hint {
                name: "Down",
                bind: app.keys.move_down.to_string(),
            },
            Hint {
                name: "Add",
                bind: app.keys.add_todo.to_string(),
            },
            Hint {
                name: "Find",
                bind: app.keys.find_mode.to_string(),
            },
            Hint {
                name: "Move",
                bind: app.keys.move_mode.to_string(),
            },
            Hint {
                name: "Toggle",
                bind: app.keys.toggle_completed.to_string(),
            },
            Hint {
                name: "Edit",
                bind: app.keys.edit_todo.to_string(),
            },
            Hint {
                name: "Delete",
                bind: app.keys.remove_todo.to_string(),
            },
            Hint {
                name: "Quit",
                bind: app.keys.quit.to_string(),
            },
        ];
        Self { hints }
    }
    pub fn edit_mode(app: &App) -> Self {
        let hints = vec![
            Hint {
                name: "Back",
                bind: app.keys.back.to_string(),
            },
            Hint {
                name: "Edit desc",
                bind: app.keys.add_description.to_string(),
            },
            Hint {
                name: "Mark recurring",
                bind: app.keys.mark_recurring.to_string(),
            },
            Hint {
                name: "Save",
                bind: app.keys.submit.to_string(),
            },
        ];
        Self { hints }
    }
    pub fn find_mode(app: &App) -> Self {
        let hints = vec![
            Hint {
                name: "Back",
                bind: app.keys.back.to_string(),
            },
            Hint {
                name: "Up",
                bind: app.keys.alt_move_up.to_string(),
            },
            Hint {
                name: "Down",
                bind: app.keys.alt_move_down.to_string(),
            },
            Hint {
                name: "Select",
                bind: app.keys.submit.to_string(),
            },
        ];
        Self { hints }
    }

    pub fn move_mode(app: &App) -> Self {
        let hints = vec![
            Hint {
                name: "Save",
                bind: app.keys.submit.to_string(),
            },
            Hint {
                name: "Up",
                bind: app.keys.move_up.to_string(),
            },
            Hint {
                name: "Down",
                bind: app.keys.move_down.to_string(),
            },
        ];
        Self { hints }
    }
}

impl Component for HintBar {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let rect = Rect {
            x: 0,
            y: size.height - 1,
            width: size.width,
            height: 1,
        };
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        for hint in self.hints.iter() {
            spans.push(Span::styled(
                format!("{} [{}]", hint.name, hint.bind),
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ));
            // space between hints
            spans.push(Span::raw(" "));
        }
        let bind_bar = Paragraph::new(Spans::from(spans))
            .wrap(tui::widgets::Wrap { trim: false })
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(bind_bar, rect);
    }
}
