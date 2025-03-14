use sample_fhir_server::routes::app_routes;
use sample_fhir_server::{
    configuration::{init_db, MongoCollections},
    state::AppState,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let collections: MongoCollections = init_db().await;

    let app_state = AppState {
        patients: Arc::new(collections.patient),
        practitioners: Arc::new(collections.practitioner),
    };

    let app = app_routes().await.with_state(app_state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
