use crate::{keys::keymap::SharedKeyList, theme::theme::SharedTheme};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::Widget,
};

#[repr(usize)]
pub enum BarType {
    Normal,
    Edit,
    Move,
    Find,
    DueDate,
}

pub struct HintBar {
    hints: Vec<Hint>,
    theme: SharedTheme,
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
    pub fn normal_mode(keys: SharedKeyList, theme: SharedTheme) -> Self {
        let hints = vec![
            Hint {
                name: "Up",
                bind: keys.move_up.to_string(),
            },
            Hint {
                name: "Down",
                bind: keys.move_down.to_string(),
            },
            Hint {
                name: "Add",
                bind: keys.add_todo.to_string(),
            },
            Hint {
                name: "Find",
                bind: keys.find_mode.to_string(),
            },
            Hint {
                name: "Move",
                bind: keys.move_mode.to_string(),
            },
            Hint {
                name: "Toggle",
                bind: keys.toggle_completed.to_string(),
            },
            Hint {
                name: "Edit",
                bind: keys.edit_todo.to_string(),
            },
            Hint {
                name: "Delete",
                bind: keys.remove_todo.to_string(),
            },
            Hint {
                name: "Desc Up",
                bind: keys.desc_scroll_up.to_string(),
            },
            Hint {
                name: "Desc Down",
                bind: keys.desc_scroll_down.to_string(),
            },
            Hint {
                name: "Quit",
                bind: keys.quit.to_string(),
            },
        ];
        Self { hints, theme }
    }
    pub fn edit_mode(keys: SharedKeyList, theme: SharedTheme) -> Self {
        let hints = vec![
            Hint {
                name: "Back",
                bind: keys.back.to_string(),
            },
            Hint {
                name: "Edit desc",
                bind: keys.external_editor.to_string(),
            },
            Hint {
                name: "Mark recurring",
                bind: keys.mark_recurring.to_string(),
            },
            Hint {
                name: "Due date",
                bind: keys.open_calendar.to_string(),
            },
            Hint {
                name: "Save",
                bind: keys.submit.to_string(),
            },
        ];
        Self { hints, theme }
    }
    pub fn find_mode(keys: SharedKeyList, theme: SharedTheme) -> Self {
        let hints = vec![
            Hint {
                name: "Back",
                bind: keys.back.to_string(),
            },
            Hint {
                name: "Up",
                bind: keys.alt_move_up.to_string(),
            },
            Hint {
                name: "Down",
                bind: keys.alt_move_down.to_string(),
            },
            Hint {
                name: "Select",
                bind: keys.submit.to_string(),
            },
        ];
        Self { hints, theme }
    }

    pub fn move_mode(keys: SharedKeyList, theme: SharedTheme) -> Self {
        let hints = vec![
            Hint {
                name: "Save",
                bind: keys.submit.to_string(),
            },
            Hint {
                name: "Move up",
                bind: keys.move_up.to_string(),
            },
            Hint {
                name: "Move down",
                bind: keys.move_down.to_string(),
            },
        ];
        Self { hints, theme }
    }

    pub fn due_date_mode(keys: SharedKeyList, theme: SharedTheme) -> Self {
        let hints = vec![
            Hint {
                name: "Select",
                bind: keys.submit.to_string(),
            },
            Hint {
                name: "Up",
                bind: keys.move_up.to_string(),
            },
            Hint {
                name: "Down",
                bind: keys.move_down.to_string(),
            },
            Hint {
                name: "Left",
                bind: keys.move_left.to_string(),
            },
            Hint {
                name: "Right",
                bind: keys.move_right.to_string(),
            },
            Hint {
                name: "Swap focus",
                bind: keys.alt_move_down.to_string(),
            },
            Hint {
                name: "Next month",
                bind: keys.alt_move_right.to_string(),
            },
            Hint {
                name: "Prev month",
                bind: keys.alt_move_left.to_string(),
            },
        ];
        Self { hints, theme }
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

impl Widget for &HintBar {
    fn render(self, rect: Rect, buf: &mut Buffer) {
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
                    .bg(self.theme.key_hint_bg)
                    .fg(self.theme.key_hint_fg)
                    .add_modifier(Modifier::BOLD),
            );
            offset_x += hl + 1;
        }
    }
}
