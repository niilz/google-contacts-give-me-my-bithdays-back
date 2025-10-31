#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Calendar {
    name: String,
    id: String,
    entries: Vec<Birthday>,
}
