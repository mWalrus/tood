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
}

impl Default for TodoMetadata {
    fn default() -> Self {
        TodoMetadata {
            added_at: Local::now(),
            edited_at: None,
        }
    }
}

impl From<&str> for TodoMetadata {
    fn from(other: &str) -> Self {
        TodoMetadata {
            added_at: DateTime::parse_from_rfc3339(other).unwrap().into(),
            ..Default::default()
        }
    }
}

impl IntoIterator for TodoMetadata {
    type Item = (&'static str, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut c = vec![];
        c.push(("Added: ", self.added_at.format(TIME_FORMAT).to_string()));

        if let Some(edited_at) = self.edited_at {
            c.push(("Edited: ", edited_at.format(TIME_FORMAT).to_string()))
        }

        c.into_iter()
    }
}
