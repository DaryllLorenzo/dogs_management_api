use axum::{
    Json, Router, extract::{Path, State}, http::StatusCode, routing::{post, put}
};
use std::sync::Arc;

use super::{
    model::{Breed, BreedPayload},
    service::BreedService,
};

pub fn breed_router(service: Arc<BreedService>) -> Router {
    Router::new()
    .route("/", post(create_breed).get(list_breeds))
    .route("/{id}",
    put(update_breed)
    .get(get_breed))
    //    .delete(delete_breed)
    //)
    .with_state(service)
}


// Get breeds - all
async fn list_breeds(
    State(service): State<Arc<BreedService>>
) -> Result<Json<Vec<Breed>>, StatusCode> {
    service.list_breeds()
    .await
    .map(Json)
    .map_err(|_| {
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn create_breed(
    State(service): State<Arc<BreedService>>,
    Json(new_breed): Json<BreedPayload>
) -> Result<(StatusCode, Json<Breed>), StatusCode> {
    service.create_breed(new_breed)
    .await
    .map(|breed| (StatusCode::CREATED, Json(breed)))
    .map_err(|_| {
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn update_breed(
    State(service): State<Arc<BreedService>>,
    Path(id): Path<i32>,
    Json(update_data): Json<BreedPayload>
) -> Result<Json<Breed>, StatusCode> {
    service.update_breed(id, update_data)
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_breed(
    State(service): State<Arc<BreedService>>,
    Path(id): Path<i32>
) -> Result<Json<Breed>, StatusCode>{
    service.get_breed(id)
    .await
    .map(|breed| Json(breed))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}