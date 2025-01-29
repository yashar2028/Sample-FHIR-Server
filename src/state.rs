use std::sync::Arc;
use mongodb::Collection;
// use fhirbolt::model::r5::resources::{Patient, Practitioner};
use serde::{Deserialize, Serialize};
// use fhir_rs::prelude::*;
use fhir_sdk::r5::resources::{Patient, Practitioner};

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoPatient {
    #[serde(flatten)] // This tells serde to inline the fields of the resource into Self, essentially making them the same structure while enabling Serialize/Deserialize.
    pub patient: Patient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoPractitioner {
    #[serde(flatten)]
    pub practitioner: Practitioner,
}

#[derive(Clone)]
pub struct AppState {
    pub patients: Arc<Collection<MongoPatient>>,
    pub practitioners: Arc<Collection<MongoPractitioner>>,
}
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppState {
    pub patients: Arc<Collection<Patient>>,
    pub practitioners: Arc<Collection<Practitioner>>
}
