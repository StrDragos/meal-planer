use crate::common::errors::db_error::DbError;
use crate::userdata::domain::diet::Diet;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait DietRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Diet>, DbError>;
}

pub struct DietRepositoryPostgres {
    postgres_pool: Arc<PgPool>,
}

#[async_trait]
impl DietRepository for DietRepositoryPostgres {
    async fn get_all(&self) -> Result<Vec<Diet>, DbError> {
        sqlx::query_as::<_, DietDb>("SELECT * FROM diets")
            .fetch_all(&*self.postgres_pool)
            .await
            .map_err(|er| DbError::from(er))
            .map(|x| x.into_iter().map(|v| v.into()).collect::<Vec<Diet>>())
    }
}

#[derive(Debug, FromRow)]
struct DietDb {
    pub id: i32,
    pub name: String,
}
impl From<DietDb> for Diet {
    fn from(value: DietDb) -> Self {
        Diet {
            id: value.id,
            name: value.name,
        }
    }
}

pub struct DietRepoBuilder;
impl DietRepoBuilder {
    pub fn postgres(postgres_pool: Arc<PgPool>) -> DietRepositoryPostgres {
        DietRepositoryPostgres { postgres_pool }
    }
}
