use super::utils::{serialize_local_date, serialize_optional_local_date};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

static TIME_FORMAT: &'static str = "%D %-I:%M %P";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoMetadata {
    #[serde(serialize_with = "serialize_local_date")]
    pub added_at: DateTime<Local>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_local_date"
    )]
    pub edited_at: Option<DateTime<Local>>,
    pub recurring: bool,
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

impl TodoMetadata {
    pub fn formatted_vec(self) -> Vec<(&'static str, String)> {
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
