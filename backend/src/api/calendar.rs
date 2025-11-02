use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct CalendarList {
    pub(crate) items: Vec<Calendar>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Calendar {
    pub(crate) etag: String,
    pub(crate) id: String,
    pub(crate) summary: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub(crate) struct InsertEvent {
    pub(crate) summary: String,
    pub(crate) start: Date,
    pub(crate) end: Date,
    #[serde(rename = "eventType")]
    // cannot be birthday (those are only allowed on primary calendar)
    pub(crate) event_type: String,
    pub(crate) transparency: String,
    pub(crate) visibility: String,
    pub(crate) recurrence: Vec<String>,
    // TODO: reminders
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub(crate) struct Date {
    // format yyyy-mm-dd
    pub(crate) date: String,
}

impl From<&str> for Date {
    fn from(date: &str) -> Self {
        Self {
            date: date.to_string(),
        }
    }
}
