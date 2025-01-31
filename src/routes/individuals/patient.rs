use crate::state::{AppState, BsonPatient};
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use mongodb::bson::doc;
use uuid::Uuid;
use fhir_sdk::TryStreamExt; // Required trait for .try_collect() method which is a function provided by this trait (See their documentation).

pub fn patient_routes() -> Router<AppState> {
    Router::new()
        .route("/fhir/Patient", post(create_patient).get(get_patients))
        .route("/fhir/Patient/{id}", get(get_patient_with_id))
}

pub async fn create_patient(
    State(state): State<AppState>,
    Json(mut patient): Json<BsonPatient>,
) -> Json<BsonPatient> {
    if patient.patient.0.id.is_none() { // New id is generated if the id field is not proided. Check the Patient struct in fhir_sdk documentation to realize the path. BsonPatient.patient.0.id. 0 leads to
        patient.patient.0.id = Some(Uuid::new_v4().to_string()); // PatientInner struct which is warped by Patient struct (pub struct Patient(pub Box<PatientInner>)) and we take id from there.  
    }

    let patient_collection = &state.patients;

    patient_collection
        .insert_one(patient.clone())
        .await
        .unwrap();

    Json(patient)
}

pub async fn get_patients(State(state): State<AppState>) -> Json<Vec<BsonPatient>> {
    let patient_collection = &state.patients;

    let cursor = patient_collection.find(doc! {}).await.unwrap(); // Fetch all the resources (no filter).
    let patients: Vec<BsonPatient> = cursor.try_collect().await.unwrap();

    Json(patients)
}

pub async fn get_patient_with_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Option<BsonPatient>> {
    let patient_collection = &state.patients;
    let filter = doc! { "id": id };

    let patient = patient_collection.find_one(filter).await.unwrap();

    Json(patient)
}
