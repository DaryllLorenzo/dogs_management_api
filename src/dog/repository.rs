use std::sync::Arc;

use sqlx::{PgPool, Error};
use super::model::{Dog, DogPayload, DogPatchPayload};

pub struct DogRepository {
    pool: Arc<PgPool>,
}

impl DogRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Dog>, Error> {
        sqlx::query_as::<_, Dog>("SELECT * FROM dogs ORDER BY id")
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Dog>, Error> {
        sqlx::query_as::<_, Dog>("SELECT * FROM dogs WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn create(&self, payload: &DogPayload) -> Result<Dog, Error> {
        sqlx::query_as::<_, Dog>(
            "INSERT INTO dogs (name, age) VALUES ($1, $2) RETURNING *"
        )
        .bind(&payload.name)
        .bind(payload.age)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn update_full(&self, id: i32, payload: &DogPayload) -> Result<Dog, Error> {
        sqlx::query_as::<_, Dog>(
            "UPDATE dogs SET name = $1, age = $2 WHERE id = $3 RETURNING *"
        )
        .bind(&payload.name)
        .bind(payload.age)
        .bind(id)
        .fetch_one(&*self.pool)
            .await
    }

    pub async fn update_partial(&self, id: i32, payload: &DogPatchPayload) -> Result<Dog, Error> {
        let current_dog = self.find_by_id(id).await?.ok_or(Error::RowNotFound)?;
        
        let name = payload.name.as_ref().unwrap_or(&current_dog.name);
        let age = payload.age.unwrap_or(current_dog.age);
        
        sqlx::query_as::<_, Dog>(
            "UPDATE dogs SET name = $1, age = $2 WHERE id = $3 RETURNING *"
        )
        .bind(name)
        .bind(age)
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn delete(&self, id: i32) -> Result<bool, Error> {
        let result = sqlx::query("DELETE FROM dogs WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
}