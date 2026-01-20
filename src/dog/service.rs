use std::sync::Arc;
use sqlx::Error;
use super::{
    model::{Dog, DogPayload, DogPatchPayload},
    repository::DogRepository,
};

pub struct DogService {
    repository: Arc<DogRepository>,
}

impl DogService {
    pub fn new(repository: Arc<DogRepository>) -> Self {
        Self { repository }
    }

    // List all dogs with optional pagination
    pub async fn list_dogs(&self, page: Option<u32>, limit: Option<u32>) -> Result<Vec<Dog>, Error> {
        let dogs = self.repository.find_all().await?;
        
        // Implement simple in-memory pagination
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(20);
        let start = ((page - 1) * limit) as usize;
        let end = (page * limit) as usize;
        
        Ok(dogs.into_iter().skip(start).take(end - start).collect())
    }

    // Get a specific dog by ID
    pub async fn get_dog(&self, id: i32) -> Result<Dog, String> {
        self.repository.find_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Dog with ID {} not found", id))
    }

    // Create a new dog with validation
    pub async fn create_dog(&self, payload: DogPayload) -> Result<Dog, String> {
        // Business logic validation
        if payload.name.trim().is_empty() {
            return Err("Dog name cannot be empty".to_string());
        }
        
        if payload.age < 0 || payload.age > 30 {
            return Err("Dog age must be between 0 and 30".to_string());
        }
        
        self.repository.create(&payload).await
            .map_err(|e| e.to_string())
    }

    // Full update (PUT)
    pub async fn update_dog_full(&self, id: i32, payload: DogPayload) -> Result<Dog, String> {
        // Verify dog exists
        let _ = self.get_dog(id).await?;
        
        // Apply business rules
        if payload.name.trim().is_empty() {
            return Err("Dog name cannot be empty".to_string());
        }
        
        self.repository.update_full(id, &payload).await
            .map_err(|e| e.to_string())
    }

    // Partial update (PATCH)
    pub async fn update_dog_partial(&self, id: i32, payload: DogPatchPayload) -> Result<Dog, String> {
        
        // Validate partial update
        if let Some(name) = &payload.name {
            if name.trim().is_empty() {
                return Err("Dog name cannot be empty".to_string());
            }
        }
        
        if let Some(age) = payload.age {
            if age < 0 || age > 30 {
                return Err("Dog age must be between 0 and 30".to_string());
            }
        }
        
        self.repository.update_partial(id, &payload).await
            .map_err(|e| e.to_string())
    }

    // Delete a dog
    pub async fn delete_dog(&self, id: i32) -> Result<bool, String> {
        // Verify dog exists
        let _ = self.get_dog(id).await?;
        
        self.repository.delete(id).await
            .map_err(|e| e.to_string())
    }
    
}