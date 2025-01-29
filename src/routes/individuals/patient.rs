use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use fhir_sdk::r5::resources::{Patient, Practitioner};
use mongodb::bson::doc;
use uuid::Uuid;

pub fn patient_routes() -> Router<AppState> {
    Router::new()
        .route("/fhir/Patient", post(create_patient).get(get_patients))
        .route("/fhir/Patient/:id", get(get_patient_with_id))
}

pub async fn create_patient(
    State(state): State<AppState>,
    Json(mut patient): Json<Patient>,
) -> Json<Patient> {
    if patient.id.is_none() { // New id is generated if the id field is not proided.
        patient.id = Some(Uuid::new_v4().to_string());
    }

    let patient_collection = &state.patients;

    patient_collection
        .insert_one(patient.clone())
        .await
        .unwrap();

    Json(patient)
}

pub async fn get_patients(State(state): State<AppState>) -> Json<Vec<Patient>> {
    let patient_collection = &state.patients;

    let cursor = patient_collection.find(None).await.unwrap(); // Fetch all the resources (no filter).
    let patients: Vec<Patient> = cursor.try_collect().await.unwrap();

    Json(patients)
}

pub async fn get_patient_with_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Option<Patient>> {
    let patient_collection = &state.patients;
    let filter = doc! { "id": id };

    let patient = patient_collection.find_one(filter).await.unwrap();

    Json(patient)
}
