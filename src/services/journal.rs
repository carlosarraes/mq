use std::collections::HashMap;

use chrono::NaiveDate;

use crate::dao::journal::JournalDao;
use crate::models::dev_journal::{DevJournal, ProjectEntry, StatusEntry, TaskEntry};
use crate::models::journal::{Journal, JournalDto};

pub struct JournalService {
    dao: JournalDao,
}

impl JournalService {
    pub fn new(dao: JournalDao) -> Self {
        Self { dao }
    }

    async fn transform_to_structure(journals: Vec<Journal>) -> DevJournal {
        let mut project_map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

        for journal in &journals {
            let status_map = project_map
                .entry(journal.project.clone())
                .or_insert_with(HashMap::new);
            let tasks = status_map
                .entry(journal.status.clone())
                .or_insert_with(Vec::new);

            tasks.push(journal.comment.clone());
        }

        let projects = project_map
            .into_iter()
            .map(|(name, statuses_map)| ProjectEntry {
                name,
                statuses: statuses_map
                    .into_iter()
                    .map(|(status, tasks)| StatusEntry {
                        status,
                        tasks: tasks
                            .into_iter()
                            .map(|description| TaskEntry { description })
                            .collect(),
                    })
                    .collect(),
            })
            .collect();

        DevJournal { projects }
    }

    fn dev_journal_to_markdown(dev_journal: &DevJournal) -> String {
        let mut markdown_content = String::new();

        for project in &dev_journal.projects {
            markdown_content += &format!("### {}\n\n", project.name);

            for status in &project.statuses {
                markdown_content += &format!("- **{}**\n", status.status);

                for task in &status.tasks {
                    markdown_content += &format!("    - {}\n", task.description);
                }

                markdown_content += "\n";
            }

            markdown_content += "\n\n";
        }

        markdown_content
    }

    fn serialize_dev_journal_to_toml(dev_journal: DevJournal) -> String {
        toml::to_string(&dev_journal).expect("Failed to serialize dev journal to TOML")
    }

    fn serialize_dev_journal_to_yaml(dev_journal: DevJournal) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&dev_journal)
    }

    fn serialize_dev_journal_to_md(dev_journal: DevJournal) -> String {
        Self::dev_journal_to_markdown(&dev_journal)
    }

    pub async fn create(&self, insert_journal: JournalDto) -> Result<(), sqlx::Error> {
        self.dao.create(insert_journal).await
    }

    pub async fn get_all(&self) -> Result<Vec<Journal>, sqlx::Error> {
        self.dao.get_all().await
    }

    pub async fn get_all_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Journal>, sqlx::Error> {
        self.dao.get_all_by_date_range(start_date, end_date).await
    }

    pub async fn get_all_by_dev_id_and_date_range(
        &self,
        dev_id: i64,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Journal>, sqlx::Error> {
        self.dao
            .get_all_by_dev_id_and_date_range(dev_id, start_date, end_date)
            .await
    }

    pub async fn get_all_by_project_id(
        &self,
        project_id: i64,
    ) -> Result<Vec<Journal>, sqlx::Error> {
        self.dao.get_all_by_project_id(project_id).await
    }

    pub async fn serialize_to_toml(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<String, sqlx::Error> {
        let journals = self.get_all_by_date_range(start_date, end_date).await?;
        let dev_journal = Self::transform_to_structure(journals).await;
        Ok(Self::serialize_dev_journal_to_toml(dev_journal))
    }

    pub async fn serialize_to_yaml(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<String, sqlx::Error> {
        let journals = self.get_all_by_date_range(start_date, end_date).await?;
        let dev_journal = Self::transform_to_structure(journals).await;
        Ok(Self::serialize_dev_journal_to_yaml(dev_journal).unwrap())
    }

    pub async fn serialize_to_md(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<String, sqlx::Error> {
        let journals = self.get_all_by_date_range(start_date, end_date).await?;
        let dev_journal = Self::transform_to_structure(journals).await;
        Ok(Self::serialize_dev_journal_to_md(dev_journal))
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
