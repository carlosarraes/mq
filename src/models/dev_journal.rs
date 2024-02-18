use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct DevJournal {
    pub dev_name: String,
    pub entries: Vec<Entry>,
}

#[derive(Serialize)]
pub struct Entry {
    pub date: NaiveDate,
    pub projects: Vec<ProjectEntry>,
}

#[derive(Serialize)]
pub struct ProjectEntry {
    pub name: String,
    pub tasks: Vec<TaskEntry>,
}

#[derive(Serialize)]
pub struct TaskEntry {
    pub description: String,
}
