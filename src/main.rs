mod dog;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::{env, sync::Arc};
use sqlx::postgres::PgPoolOptions;
use dog::{DogRepository, DogService, dog_router};

#[tokio::main]
async fn main() {
   
    // 1. Load configuration
    dotenv().expect("Failed to load .env file");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 2. Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

    // 3. Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failed");

    // 4. Create repository and service
    let repository = Arc::new(DogRepository::new(pool));
    let service = Arc::new(DogService::new(repository));

    // 5. Create main router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api/dogs", dog_router(service));

    // 6. Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to Dog Management API!"
}

async fn health_check() -> &'static str {
    "OK"
}