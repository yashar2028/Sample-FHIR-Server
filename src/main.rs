use Sample_FHIR_Server::{
    configuration::{MongoCollections, init_db}, 
    state::AppState,};
use Sample_FHIR_Server::routes::app_routes;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let collections: MongoCollections = init_db().await;

    let app_state = AppState {
        patients: Arc::new(collections.patient),
        practitioners: Arc::new(collections.practitioner)
    };

    let app = app_routes().with_state(app_state);
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
}
