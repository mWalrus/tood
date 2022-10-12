use super::{utils, Component};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem},
    Frame,
};

static TIME_FORMAT: &str = "%D %-I:%M %P";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoMetadata {
    pub added_at: DateTime<Local>,
    pub edited_at: Option<DateTime<Local>>,
    pub recurring: bool,
}

impl TodoMetadata {
    fn to_formatted(&self) -> Vec<(&'static str, String)> {
        #[inline(always)]
        fn yes_no(b: bool) -> &'static str {
            if b {
                "yes"
            } else {
                "no"
            }
        }

        let mut c = vec![];
        c.push(("Added: ", self.added_at.format(TIME_FORMAT).to_string()));

        let edited_at = if let Some(ea) = self.edited_at {
            ea.format(TIME_FORMAT).to_string()
        } else {
            String::new()
        };

        c.push(("Edited: ", edited_at));
        c.push(("Recurring: ", yes_no(self.recurring).into()));
        c
    }
}

impl Default for TodoMetadata {
    fn default() -> Self {
        TodoMetadata {
            added_at: Local::now(),
            edited_at: None,
            recurring: false,
        }
    }
}

impl Component for TodoMetadata {
    fn draw_in_rect<B: Backend>(&self, f: &mut Frame<B>, r: &Rect) {
        let mut list_items: Vec<ListItem> = Vec::new();
        for md in self.to_formatted() {
            let spans = Spans::from(vec![
                Span::styled(md.0, Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(md.1.to_string()),
            ]);
            list_items.push(ListItem::new(spans));
        }
        let list = List::new(list_items).block(utils::default_block("Metadata"));
        f.render_widget(list, *r);
    }
}
