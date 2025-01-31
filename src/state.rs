use std::sync::Arc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use fhir_sdk::r5::resources::{Patient, Practitioner};

#[derive(Clone)]
pub struct AppState {
    pub patients: Arc<Collection<BsonPatient>>, // Type Collection<T> requires that T implements Serialize and Deserialize (to serialize/deserialize to/from bson (Mongo storage format)) and apperantly the
    pub practitioners: Arc<Collection<BsonPractitioner>> // resources from fhir_sdk do not implement these directly (but they do implement serde for Json), so we wrap them into a struct 
                                                         // and derive serde on them.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsonPatient {
    #[serde(flatten)]
    pub patient: Patient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsonPractitioner {
    #[serde(flatten)]
    pub practitioner: Practitioner,
}
