use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

use super::app::App;

pub struct HintBar {
    hints: Vec<Hint>,
}

pub struct Hint {
    name: &'static str,
    bind: String,
}

impl From<&Hint> for String {
    fn from(other: &Hint) -> Self {
        format!("{} [{}]", other.name, other.bind)
    }
}

impl Hint {
    fn len(&self) -> usize {
        String::from(self).chars().count()
    }
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

    pub fn height_required(&self, width: u16, height: u16) -> u16 {
        let (mut x, mut y) = (0u16, 1u16);
        for hint in self.hints.iter() {
            // dont extend height to infinity
            if y == height {
                break;
            }
            let hl = hint.len() as u16;
            if x + hl + 1 > width {
                x = 0;
                y += 1;
            } else {
                x += hl + 1;
            }
        }
        y
    }
}

impl Widget for HintBar {
    fn render(self, rect: Rect, buf: &mut Buffer) {
        // TODO
        let (mut offset_x, mut offset_y) = (rect.x, rect.y);
        for hint in self.hints.iter() {
            let hl = hint.len() as u16;
            if offset_x + hl > rect.width {
                offset_y += 1;
                offset_x = rect.x;
            }

            // max height reached, stop rendering
            if offset_y == rect.y + rect.height {
                break;
            }

            buf.set_string(
                offset_x,
                offset_y,
                String::from(hint),
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );
            offset_x += hl + 1;
        }
    }
}
