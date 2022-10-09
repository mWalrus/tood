use chrono::{DateTime, Local};
use serde::Serializer;

pub fn serialize_local_date<S>(dt: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.to_rfc3339())
}

pub fn serialize_optional_local_date<S>(
    dt: &Option<DateTime<Local>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.unwrap().to_rfc3339())
}
