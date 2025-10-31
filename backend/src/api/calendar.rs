use serde::Deserialize;

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
