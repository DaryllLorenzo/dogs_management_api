use axum::{
    Router,
    routing::{get, post},
    extract::{Path, State, Query},
    Json,
    http::StatusCode,
};
use std::sync::Arc;
use super::{
    model::{Dog, DogPayload, DogPatchPayload},
    service::DogService,
};
use serde::Deserialize;

// Query parameters for pagination
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// This function receives the service and returns a configured router
pub fn dog_router(service: Arc<DogService>) -> Router {
    Router::new()
        .route("/", post(create_dog).get(list_dogs))
        .route("/{id}", 
            get(get_dog)
            .put(update_dog_put)
            .patch(update_dog_patch)
            .delete(delete_dog)
        )
        .with_state(service)
}

// GET /dogs - List all dogs with pagination
async fn list_dogs(
    State(service): State<Arc<DogService>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Dog>>, StatusCode> {
    service.list_dogs(params.page, params.limit)
        .await
        .map(Json)
        .map_err(|_| {
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

// POST /dogs - Create a new dog
async fn create_dog(
    State(service): State<Arc<DogService>>,
    Json(new_dog): Json<DogPayload>,
) -> Result<(StatusCode, Json<Dog>), StatusCode> {
    service.create_dog(new_dog)
        .await
        .map(|dog| (StatusCode::CREATED, Json(dog)))
        .map_err(|e| {
            
            // Distinguish between validation errors and server errors
            if e.to_string().contains("cannot be empty") || 
               e.to_string().contains("must be between") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}

// GET /dogs/{id} - Get a specific dog by ID
async fn get_dog(
    State(service): State<Arc<DogService>>,
    Path(id): Path<i32>,
) -> Result<Json<Dog>, StatusCode> {
    service.get_dog(id)
        .await
        .map(Json)
        .map_err(|_| {
            StatusCode::NOT_FOUND
        })
}

// PUT /dogs/{id} - Full update
async fn update_dog_put(
    State(service): State<Arc<DogService>>,
    Path(id): Path<i32>,
    Json(update_data): Json<DogPayload>,
) -> Result<Json<Dog>, StatusCode> {
    service.update_dog_full(id, update_data)
        .await
        .map(Json)
        .map_err(|e| {
            
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else if e.to_string().contains("cannot be empty") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}

// PATCH /dogs/{id} - Partial update
async fn update_dog_patch(
    State(service): State<Arc<DogService>>,
    Path(id): Path<i32>,
    Json(patch_data): Json<DogPatchPayload>,
) -> Result<Json<Dog>, StatusCode> {
    service.update_dog_partial(id, patch_data)
        .await
        .map(Json)
        .map_err(|e| {
            
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else if e.to_string().contains("cannot be empty") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}

// DELETE /dogs/{id} - Delete a dog by ID
async fn delete_dog(
    State(service): State<Arc<DogService>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    service.delete_dog(id)
        .await
        .map_err(|e| {
            
            if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
        .and_then(|deleted| {
            if deleted {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        })
}