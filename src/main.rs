use std::env;
use dotenvy::dotenv;
use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, post}};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};

// In DTO
#[derive(Deserialize)]
struct DogPayLoad {
    name : String,
    age : i32
}

// Out DTO
#[derive(Serialize, FromRow)]
struct Dog {
    id: i32,
    name : String,
    age : i32
}

#[tokio::main]
async fn main () {
    dotenv().expect("Failed to load .env file");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool."); // try to connect to db

    sqlx::migrate!().run(&pool).await.expect("Migration failed");

    
    let app = Router::new()
    .route("/", get(root))
    .route("/dogs", post(create_dog).get(list_dogs))
    .route("/dogs/{id}", get(get_dog).put(update_dog).delete(delete_dog))
    .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on port 8000");
    axum::serve(listener, app).await.unwrap();
}


// Endpoints Handlers
//test endpoint
async fn root() -> &'static str{
    "Welcome to Dog Management API!"
}

// GET ALL
async fn list_dogs(
    State(pool): State<PgPool>
    ) -> Result<Json<Vec<Dog>>, StatusCode> {
    sqlx::query_as::<_, Dog>("SELECT * FROM dogs")
        .fetch_all(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// CREATE DOG
//#[axum::debug_handler] for debug!
async fn create_dog(
    State(pool): State<PgPool>,
    Json(new_dog) : Json<DogPayLoad>
    ) -> Result<(StatusCode, Json<Dog>), StatusCode> {
    
    sqlx::query_as::<_, Dog>("INSERT INTO dogs (name, age) VALUES ($1 , $2) RETURNING *")
        .bind(new_dog.name)
        .bind(new_dog.age)
        .fetch_one(&pool).await
        .map(|d| (StatusCode::CREATED, Json(d)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// GET DOG BY ID
async fn get_dog(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
    ) -> Result<Json<Dog>, StatusCode> {

    sqlx::query_as::<_, Dog>("SELECT * FROM dogs WHERE id = $1")
    .bind(id)
    .fetch_one(&pool).await
    .map(Json)
    .map_err(|_| StatusCode::NOT_FOUND)
}

// UPDATE DOG
async fn update_dog(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update_data): Json<DogPayLoad>
    ) -> Result<Json<Dog>, StatusCode> {
    
    sqlx::query_as::<_, Dog>("UPDATE dogs SET name = $1, age = $2 WHERE id = $3 RETURNING *")
    .bind(update_data.name)
    .bind(update_data.age)
    .bind(id)
    .fetch_one(&pool).await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// DELETE DOG
async fn delete_dog(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
    ) -> Result<StatusCode, StatusCode> {
    
    let result = sqlx::query("DELETE FROM dogs WHERE id = $1")
    .bind(id)
    .execute(&pool).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}