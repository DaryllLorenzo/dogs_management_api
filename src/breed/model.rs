use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Input DTO for CREATE and PUT (full update)
#[derive(Deserialize)]
pub struct BreedPayload {
    pub name: String
}

// Output DTO for READ operations
#[derive(Serialize, FromRow)]
pub struct Breed {
    pub id: i32,
    pub name: String
}