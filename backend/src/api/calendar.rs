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

#[derive(Serialize, Debug, PartialEq, Eq)]
pub(crate) struct InsertEvent {
    pub(crate) summary: String,
    pub(crate) start: String,
    pub(crate) end: String,
    #[serde(rename = "birthdayProperties")]
    pub(crate) birthday_properties: BirthdayProperties,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub(crate) struct BirthdayProperties {
    //pub(crate) contact: String, format people/c12345
    #[serde(rename = "type")]
    pub(crate) typ: String,
}
