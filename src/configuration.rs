use mongodb::{Client, Collection, Database};
use fhir_sdk::r5::resources::{Patient, Practitioner};

pub struct MongoCollections {
    pub patient: Collection<Patient>,
    pub practitioner: Collection<Practitioner>
}

pub async fn init_db() -> MongoCollections {
    let client: Client = Client::with_uri_str("mongodb://user1:12345678910@localhost:27017/server_resources")
        .await
        .expect("Failed to initialize client");

    let db: Database = client.database("server_resources");

    let patient_collection = db.collection::<Patient>("patients");
    let practitioner_collection = db.collection::<Practitioner>("practitioners");

    MongoCollections {
        patient: patient_collection,
        practitioner: practitioner_collection
    }
}
