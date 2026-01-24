use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Input DTO for CREATE and PUT (full update)
#[derive(Deserialize)]
pub struct DogPayload {
    pub name: String,
    pub age: i32,
    pub breed_id: Option<i32>,  // optional
}

#[derive(Deserialize)]
pub struct DogPatchPayload {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub breed_id: Option<Option<i32>>,  // Option<Option> so we can know if it's null or not send
}

#[derive(Serialize, FromRow)]
pub struct Dog {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub breed_id: Option<i32>,  // Foreign key
}

// DTO for join relation
#[derive(Debug)]
#[derive(Serialize, FromRow)]
pub struct DogWithBreed {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub breed_id: Option<i32>,
    pub breed_name: Option<String>, 
}