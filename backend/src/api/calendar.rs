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
    pub(crate) recurrence: Vec<String>,
    #[serde(rename = "eventType")]
    pub(crate) event_type: String,
    #[serde(rename = "birthdayProperties")]
    pub(crate) birthday_properties: BirthdayProperties,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub(crate) struct BirthdayProperties {
    //pub(crate) contact: String, format people/c12345
    #[serde(rename = "type")]
    pub(crate) typ: String,
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
