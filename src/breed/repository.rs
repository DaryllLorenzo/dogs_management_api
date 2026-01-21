use std::sync::Arc;

use sqlx::{PgPool, Error};
use super::model::{Breed, BreedPayload};

pub struct BreedRepository {
    pool: Arc<PgPool>,
}

impl BreedRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Breed>, Error> {
        sqlx::query_as::<_, Breed>("SELECT * FROM breeds ORDER BY id")
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn create(&self, payload: &BreedPayload) -> Result<Breed, Error> {
        sqlx::query_as::<_, Breed>(
            "INSERT INTO breeds (name) VALUES ($1) RETURNING *"
        )
        .bind(&payload.name)
        .fetch_one(&*self.pool)
        .await
    }
}