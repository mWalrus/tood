use std::rc::Rc;

use confy::ConfyError;
use serde::{Deserialize, Serialize};
use tui::style::Color;
use tui_utils::keys::Shared;

use super::theme::ToodTheme;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ThemeConfig {
    pub border: Option<Color>,
    pub move_mode_border: Option<Color>,
    pub move_mode_fg: Option<Color>,
    pub move_mode_bg: Option<Color>,
    pub section_title: Option<Color>,
    pub todo_title: Option<Color>,
    pub recurring_todo_title: Option<Color>,
    pub completed_todo_title: Option<Color>,
    pub selected_fg: Option<Color>,
    pub selected_bg: Option<Color>,
    pub scrollbar: Option<Color>,
    pub key_hint_fg: Option<Color>,
    pub key_hint_bg: Option<Color>,
    pub flash_info_fg: Option<Color>,
    pub flash_info_bg: Option<Color>,
    pub flash_warn_fg: Option<Color>,
    pub flash_warn_bg: Option<Color>,
    pub flash_err_fg: Option<Color>,
    pub flash_err_bg: Option<Color>,
}

impl ThemeConfig {
    pub fn read_from_file() -> Result<Self, ConfyError> {
        confy::load("tood", Some("theme"))
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_shared(self) -> Rc<ToodTheme> {
        let dt = ToodTheme::default();

        let theme = ToodTheme {
            border: self.border.unwrap_or(dt.border),
            move_mode_border: self.move_mode_border.unwrap_or(dt.move_mode_border),
            move_mode_fg: self.move_mode_fg.unwrap_or(dt.move_mode_fg),
            move_mode_bg: self.move_mode_bg.unwrap_or(dt.move_mode_bg),
            section_title: self.section_title.unwrap_or(dt.section_title),
            todo_title: self.todo_title.unwrap_or(dt.todo_title),
            recurring_todo_title: self.recurring_todo_title.unwrap_or(dt.recurring_todo_title),
            completed_todo_title: self.completed_todo_title.unwrap_or(dt.completed_todo_title),
            selected_fg: self.selected_fg.unwrap_or(dt.selected_fg),
            selected_bg: self.selected_bg.unwrap_or(dt.selected_bg),
            scrollbar: self.scrollbar.unwrap_or(dt.scrollbar),
            key_hint_fg: self.key_hint_fg.unwrap_or(dt.key_hint_fg),
            key_hint_bg: self.key_hint_bg.unwrap_or(dt.key_hint_bg),
            flash_info_fg: self.flash_info_fg.unwrap_or(dt.flash_info_fg),
            flash_info_bg: self.flash_info_bg.unwrap_or(dt.flash_info_bg),
            flash_warn_fg: self.flash_warn_fg.unwrap_or(dt.flash_warn_fg),
            flash_warn_bg: self.flash_warn_bg.unwrap_or(dt.flash_warn_bg),
            flash_err_fg: self.flash_err_fg.unwrap_or(dt.flash_err_fg),
            flash_err_bg: self.flash_err_bg.unwrap_or(dt.flash_err_bg),
        };

        Rc::new(theme)
    }
}
