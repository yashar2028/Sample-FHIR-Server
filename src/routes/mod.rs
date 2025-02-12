use crate::routes::individuals::patient::patient_routes;
use axum::Router;
use crate::state::AppState;

pub mod individuals;

pub async fn app_routes() -> Router<AppState> { // Combine all routes into a single router.
    Router::new()
        .merge(patient_routes())
}
