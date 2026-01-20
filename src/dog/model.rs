use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Input DTO for CREATE and PUT (full update)
#[derive(Deserialize)]
pub struct DogPayload {
    pub name: String,
    pub age: i32,
}

// Input DTO for PATCH (partial update)
#[derive(Deserialize)]
pub struct DogPatchPayload {
    pub name: Option<String>,  // Optional field
    pub age: Option<i32>,      // Optional field
}

// Output DTO for READ operations
#[derive(Serialize, FromRow)]
pub struct Dog {
    pub id: i32,
    pub name: String,
    pub age: i32,
}