use axum::{
    Router,
    routing::{get, post},
    extract::State,
    Json,
    http::StatusCode,
};
use std::sync::Arc;

use super::{
    model::{Breed, BreedPayload},
    service::BreedService,
};

pub fn breed_router(service: Arc<BreedService>) -> Router {
    Router::new()
    .route("/", post(create_breed).get(list_breeds))
    //.route("/{id}",
    //    get(get_breed)
    //    .put(update_breed)
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

