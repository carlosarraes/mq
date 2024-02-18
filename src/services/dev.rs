use crate::{
    dao::dev::DevDao,
    models::dev::{Dev, DevDto},
};

pub struct DevService {
    dev_dao: DevDao,
}

impl DevService {
    pub fn new(dev_dao: DevDao) -> Self {
        Self { dev_dao }
    }

    pub async fn create(&self, insert_dev: DevDto) -> Result<(), sqlx::Error> {
        self.dev_dao.create(insert_dev).await
    }

    pub async fn get_all(&self) -> Result<Vec<Dev>, sqlx::Error> {
        self.dev_dao.get_all().await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Dev, sqlx::Error> {
        self.dev_dao.get_by_id(id).await
    }

    pub async fn update(&self, id: i64, update_dev: DevDto) -> Result<(), sqlx::Error> {
        self.dev_dao.update(id, update_dev).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        self.dev_dao.delete(id).await
    }
}
