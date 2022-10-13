use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

static TIME_FORMAT: &str = "%D %-I:%M %P";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoMetadata {
    pub added_at: DateTime<Local>,
    pub edited_at: Option<DateTime<Local>>,
    pub recurring: bool,
}

impl TodoMetadata {
    pub fn to_formatted(&self) -> Vec<(&'static str, String)> {
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
