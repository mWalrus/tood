use super::theme::ToodTheme;
use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tui::style::Color;

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

impl Config for ThemeConfig {
    type Item = ToodTheme;

    fn to_shared(self) -> Rc<ToodTheme> {
        let dt = ToodTheme::default();

        #[rustfmt::skip]
        let theme = ToodTheme {
            border:               either!(self.border,               dt.border),
            move_mode_border:     either!(self.move_mode_border,     dt.move_mode_border),
            move_mode_fg:         either!(self.move_mode_fg,         dt.move_mode_fg),
            move_mode_bg:         either!(self.move_mode_bg,         dt.move_mode_bg),
            section_title:        either!(self.section_title,        dt.section_title),
            todo_title:           either!(self.todo_title,           dt.todo_title),
            recurring_todo_title: either!(self.recurring_todo_title, dt.recurring_todo_title),
            completed_todo_title: either!(self.completed_todo_title, dt.completed_todo_title),
            selected_fg:          either!(self.selected_fg,          dt.selected_fg),
            selected_bg:          either!(self.selected_bg,          dt.selected_bg),
            scrollbar:            either!(self.scrollbar,            dt.scrollbar),
            key_hint_fg:          either!(self.key_hint_fg,          dt.key_hint_fg),
            key_hint_bg:          either!(self.key_hint_bg,          dt.key_hint_bg),
            flash_info_fg:        either!(self.flash_info_fg,        dt.flash_info_fg),
            flash_info_bg:        either!(self.flash_info_bg,        dt.flash_info_bg),
            flash_warn_fg:        either!(self.flash_warn_fg,        dt.flash_warn_fg),
            flash_warn_bg:        either!(self.flash_warn_bg,        dt.flash_warn_bg),
            flash_err_fg:         either!(self.flash_err_fg,         dt.flash_err_fg),
            flash_err_bg:         either!(self.flash_err_bg,         dt.flash_err_bg),
        };

        Rc::new(theme)
    }
}
