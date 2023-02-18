use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tui::style::Color;
use tui_utils::keys::Shared;

use super::theme_config::ThemeConfig;

pub type SharedTheme = Rc<ToodTheme>;

#[derive(Debug, Shared, Serialize, Deserialize)]
pub struct ToodTheme {
    pub border: Color,
    pub move_mode_border: Color,
    pub move_mode_fg: Color,
    pub move_mode_bg: Color,
    pub section_title: Color,
    pub todo_title: Color,
    pub recurring_todo_title: Color,
    pub completed_todo_title: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub scrollbar: Color,
    pub key_hint_fg: Color,
    pub key_hint_bg: Color,
    pub flash_info_fg: Color,
    pub flash_info_bg: Color,
    pub flash_warn_fg: Color,
    pub flash_warn_bg: Color,
    pub flash_err_fg: Color,
    pub flash_err_bg: Color,
}

#[rustfmt::skip]
impl Default for ToodTheme {
    fn default() -> Self {
        Self {
            border: Color::Reset,
            move_mode_border: Color::Blue,
            move_mode_fg: Color::Black,
            move_mode_bg: Color::Blue,
            section_title: Color::Reset,
            todo_title: Color::Reset,
            recurring_todo_title: Color::Blue,
            completed_todo_title: Color::Green,
            selected_fg: Color::Blue,
            selected_bg: Color::Indexed(8),
            scrollbar: Color::Blue,
            key_hint_fg: Color::Black,
            key_hint_bg: Color::Blue,
            flash_info_fg: Color::Black,
            flash_info_bg: Color::Green,
            flash_warn_fg: Color::Black,
            flash_warn_bg: Color::Yellow,
            flash_err_fg: Color::Black,
            flash_err_bg: Color::Red,
        }
    }
}

impl ToodTheme {
    pub fn init() -> SharedTheme {
        match ThemeConfig::read_from_file() {
            Ok(theme) => theme.to_shared(),
            Err(e) => Self::shared(),
        }
    }
}
