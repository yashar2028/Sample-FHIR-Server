use crate::state::{AppState, BsonPatient};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use fhir_sdk::TryStreamExt; // Required trait for .try_collect() method which is a function provided by this trait (See their documentation).
use mongodb::bson::{doc, to_bson};
use uuid::Uuid;

pub fn patient_routes() -> Router<AppState> {
    Router::new()
        .route("/fhir/Patient", post(create_patient).get(get_patients))
        .route(
            "/fhir/Patient/{id}",
            get(get_patient_with_id)
                .put(update_patient)
                .delete(delete_patient),
        )
}

pub async fn create_patient(
    State(state): State<AppState>,
    Json(mut patient): Json<BsonPatient>,
) -> impl IntoResponse {
    if patient.patient.0.id.is_none() {
        // New id is generated if the id field is not proided. Check the Patient struct in fhir_sdk documentation to realize the path. BsonPatient.patient.0.id. 0 leads to
        patient.patient.0.id = Some(Uuid::new_v4().to_string()); // PatientInner struct which is warped by Patient struct (pub struct Patient(pub Box<PatientInner>)) and we take id from there.
    }

    let patient_collection = &state.patients;

    patient_collection
        .insert_one(patient.clone())
        .await
        .unwrap();

    (StatusCode::CREATED, Json(patient))
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

pub async fn update_patient(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(updated_patient): Json<BsonPatient>,
) -> impl IntoResponse {
    let patient_collection = &state.patients;
    let filter = doc! { "id": &id };

    let update_doc = doc! { "$set": to_bson(&updated_patient).unwrap() }; // Updating the altered fields without replacing the whole resource (.replace_one()). Here doc macro expects a BSON.

    patient_collection
        .update_one(filter, update_doc)
        .await
        .unwrap();

    (StatusCode::OK, Json(updated_patient))
}

pub async fn delete_patient(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let patient_collection = &state.patients;
    let filter = doc! { "id": &id };

    patient_collection.delete_one(filter).await.unwrap();

    (StatusCode::NO_CONTENT, Json("Patient deleted"))
}
