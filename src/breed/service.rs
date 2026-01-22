use sqlx::Error;
use super::{
    model::{Breed, BreedPayload},
    repository::BreedRepository,
};

pub struct BreedService {
    repository: BreedRepository,
}

impl BreedService {
    pub fn new(repository: BreedRepository) -> Self {
        Self { repository }
    }

    pub async fn list_breeds(&self) -> Result<Vec<Breed>, Error> {
        let breeds = self.repository.find_all().await?;
        
        Ok(breeds.into_iter().collect())
    }

    pub async fn create_breed(&self, payload: BreedPayload) -> Result<Breed, String> {
        // Business logic validation
        if payload.name.trim().is_empty() {
            return Err("Breed name cannot be empty".to_string());
        }
        
        self.repository.create(&payload).await
            .map_err(|e| e.to_string())
    }

    pub async fn update_breed(&self, id:i32, payload: BreedPayload) -> Result<Breed, String>{
        self.repository.update(id, &payload)
        .await
        .map_err(|e| e.to_string())
    }

}