use std::collections::HashMap;

use chrono::NaiveDate;

use crate::dao::journal::JournalDao;
use crate::models::dev_journal::{DevJournal, Entry, ProjectEntry, TaskEntry};
use crate::models::journal::{Journal, JournalDto};

pub struct JournalService {
    dao: JournalDao,
}

impl JournalService {
    pub fn new(dao: JournalDao) -> Self {
        Self { dao }
    }

    async fn transform_to_structure(journals: Vec<Journal>) -> DevJournal {
        let mut entries_map: HashMap<NaiveDate, HashMap<String, Vec<TaskEntry>>> = HashMap::new();

        for journal in &journals {
            let entry = entries_map.entry(journal.day).or_insert_with(HashMap::new);

            let project_tasks = entry
                .entry(journal.project.clone())
                .or_insert_with(Vec::new);

            project_tasks.push(TaskEntry {
                description: format!("{}: {}", journal.title, journal.comment),
            });
        }

        let entries = entries_map
            .into_iter()
            .map(|(date, projects_map)| Entry {
                date,
                projects: projects_map
                    .into_iter()
                    .map(|(name, tasks)| ProjectEntry { name, tasks })
                    .collect(),
            })
            .collect();

        let dev_name = journals.get(0).map_or(String::new(), |j| j.dev.clone());

        DevJournal { dev_name, entries }
    }

    fn serialize_dev_journal_to_toml(dev_journal: DevJournal) -> String {
        toml::to_string(&dev_journal).expect("Failed to serialize dev journal to TOML")
    }

    fn serialize_dev_journal_to_yaml(dev_journal: DevJournal) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&dev_journal)
    }

    pub async fn create(&self, insert_journal: JournalDto) -> Result<(), sqlx::Error> {
        self.dao.create(insert_journal).await
    }

    pub async fn get_all(&self) -> Result<Vec<Journal>, sqlx::Error> {
        self.dao.get_all().await
    }

    pub async fn get_all_by_dev_id(&self, dev_id: i64) -> Result<Vec<Journal>, sqlx::Error> {
        self.dao.get_all_by_dev_id(dev_id).await
    }

    pub async fn get_all_by_project_id(
        &self,
        project_id: i64,
    ) -> Result<Vec<Journal>, sqlx::Error> {
        self.dao.get_all_by_project_id(project_id).await
    }

    pub async fn serialize_to_toml(&self, dev_id: i64) -> Result<String, sqlx::Error> {
        let journals = self.get_all_by_dev_id(dev_id).await?;
        let dev_journal = Self::transform_to_structure(journals).await;
        Ok(Self::serialize_dev_journal_to_toml(dev_journal))
    }

    pub async fn serialize_to_yaml(&self, dev_id: i64) -> Result<String, sqlx::Error> {
        let journals = self.get_all_by_dev_id(dev_id).await?;
        let dev_journal = Self::transform_to_structure(journals).await;
        Ok(Self::serialize_dev_journal_to_yaml(dev_journal).unwrap())
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Journal, sqlx::Error> {
        self.dao.get_by_id(id).await
    }

    pub async fn update(&self, id: i64, update_journal: JournalDto) -> Result<(), sqlx::Error> {
        self.dao.update(id, update_journal).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        self.dao.delete(id).await
    }
}
