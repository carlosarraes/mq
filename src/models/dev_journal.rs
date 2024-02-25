use serde::Serialize;

#[derive(Serialize)]
pub struct DevJournal {
    pub projects: Vec<ProjectEntry>,
}

#[derive(Serialize)]
pub struct ProjectEntry {
    pub name: String,
    pub statuses: Vec<StatusEntry>,
}

#[derive(Serialize)]
pub struct StatusEntry {
    pub status: String,
    pub tasks: Vec<TaskEntry>,
}

#[derive(Serialize)]
pub struct TaskEntry {
    pub description: String,
}
