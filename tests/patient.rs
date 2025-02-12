use sample_fhir_server::routes::app_routes;
use sample_fhir_server::state::AppState;
use sample_fhir_server::configuration::{MongoCollections, init_db};
use axum::{http::StatusCode};
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use reqwest;

async fn test_app() -> SocketAddr {
    
    let collections: MongoCollections = init_db().await;
    
    let app_state = AppState {
        patients: Arc::new(collections.patient),
        practitioners: Arc::new(collections.practitioner),
    };
    
    let app = app_routes().await.with_state(app_state);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = axum::serve(listener, app);
    tokio::spawn(async move {
        server.await.unwrap();
    }); // Start the server as a background task

    return addr

}

#[tokio::test]
async fn test_get_patients() {

    let test_app_socket = test_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/fhir/Patient", &test_app_socket))
        .send()
        .await
        .unwrap();

    // Assert that the response status is 200 OK
    assert_eq!(response.status(), StatusCode::OK);

    // No need to shutdown the server manually. tokio::spawn() has ditached the server which handles
    // the server abortion for us
}


/*
#[tokio::test]
async fn test_create_new_patinet() {
    
    let test_app_socket = test_app().await;

    let test_patinet = r#"
    {
        "resourceType": "Patient",
        "id": "12345",
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male",
        "birthDate": "1990-01-01"
    }"#;


    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/fhir/Patient", &test_app_socket))
        .header("Content-Type", "application/fhir+json")
        .body(&test_patinet)
        .send()
        .await
        .unwrap();

    // Assert that the response status is 201 Created
    assert_eq!(response.status(), StatusCode::CREATED);

    let collections: MongoCollections = init_db().await;
    let patient_collection = &collections.patient;

    let filter = doc! { "id": "123" };
    let found_patient = patient_collection
        .find_one(filter, None)
        .await
        .unwrap();

    // Assert that the patient was found in the database and queried successfully
    assert!(found_patient.is_some());
    let patient = found_patient.unwrap();
    let id = patient.get_str("id").unwrap();

    // Assert the patient's ID is correct
    assert_eq!(id, "123");

}

*/
