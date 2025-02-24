use crate::routes::individuals::patient::patient_routes;
use crate::state::AppState;
use axum::Router;

pub mod individuals;

pub async fn app_routes() -> Router<AppState> {
    // Combine all routes into a single router.
    Router::new().merge(patient_routes())
}
