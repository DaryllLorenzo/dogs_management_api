use std::sync::Arc;

use sqlx::{PgPool, Error};
use super::model::{Dog, DogPayload, DogPatchPayload, DogWithBreed};

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
            "INSERT INTO dogs (name, age, breed_id) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&payload.name)
        .bind(payload.age)
        .bind(payload.breed_id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn update_full(&self, id: i32, payload: &DogPayload) -> Result<Dog, Error> {
        sqlx::query_as::<_, Dog>(
            "UPDATE dogs SET name = $1, age = $2, breed_id = $3 WHERE id = $4 RETURNING *"
        )
        .bind(&payload.name)
        .bind(payload.age)
        .bind(payload.breed_id)
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn update_partial(&self, id: i32, payload: &DogPatchPayload) -> Result<Dog, Error> {
        let current_dog = self.find_by_id(id).await?.ok_or(Error::RowNotFound)?;
        
        let name = payload.name.as_ref().unwrap_or(&current_dog.name);
        let age = payload.age.unwrap_or(current_dog.age);
        
        // null vs not send
        let breed_id = match &payload.breed_id {
            Some(Some(breed_id)) => Some(*breed_id),  // Cliente sent value (could be null)
            Some(None) => None,                       // Cliente sent null
            None => current_dog.breed_id,             // Cliente didn't send field
        };
        
        sqlx::query_as::<_, Dog>(
            "UPDATE dogs SET name = $1, age = $2, breed_id = $3 WHERE id = $4 RETURNING *"
        )
        .bind(name)
        .bind(age)
        .bind(breed_id)
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


    pub async fn find_all_with_breed(&self) -> Result<Vec<DogWithBreed>, Error> {
        sqlx::query_as::<_, DogWithBreed>(
            "
                SELECT d.*, b.name as breed_name 
                FROM dogs d
                LEFT JOIN breeds b ON d.breed_id = b.id
                ORDER BY d.id
                "
            )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn find_by_id_with_breed(&self, id: i32) -> Result<Option<DogWithBreed>, Error> {
        sqlx::query_as::<_, DogWithBreed>(
            "
            SELECT d.*, b.name as breed_name
            FROM dogs d
            LEFT JOIN breeds b ON d.breed_id = b.id
            WHERE d.id = $1
            "
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn find_by_breed_id(&self, breed_id: i32) -> Result<Vec<Dog>, Error> {
        sqlx::query_as::<_, Dog>(
            "SELECT * FROM dogs WHERE breed_id = $1 ORDER BY id"
        )
        .bind(breed_id)
        .fetch_all(&*self.pool)
        .await
    }
}