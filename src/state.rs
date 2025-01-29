use std::sync::Arc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use fhir_sdk::r5::resources::{Patient, Practitioner};

#[derive(Clone)]
pub struct AppState {
    pub patients: Arc<Collection<Patient>>,
    pub practitioners: Arc<Collection<Practitioner>>
}
