use sample_fhir_server::routes::app_routes;
use sample_fhir_server::{
    configuration::{init_db, MongoCollections},
    state::AppState,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {

    dotenv().ok(); // Load .env file.

    let collections: MongoCollections = init_db().await;

    let app_state = AppState {
        patients: Arc::new(collections.patient),
        practitioners: Arc::new(collections.practitioner),
    };

    let app = app_routes().await
        .with_state(app_state)
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true))); // Having header info to also be included in spans.


    let tracing_layer_formatter = tracing_subscriber::fmt::layer(); // Basic formatter for logs.

    tracing_subscriber::registry()
        .with(tracing_layer_formatter)
        .with(EnvFilter::from_default_env())
        .init();


    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
