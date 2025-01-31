use mongodb::{Client, Collection, Database};
use crate::state::{BsonPatient, BsonPractitioner};

pub struct MongoCollections {
    pub patient: Collection<BsonPatient>,
    pub practitioner: Collection<BsonPractitioner>
}

pub async fn init_db() -> MongoCollections {
    let client: Client = Client::with_uri_str("mongodb://user1:12345678910@localhost:27017/server_resources")
        .await
        .expect("Failed to initialize client");

    let db: Database = client.database("server_resources");

    let patient_collection = db.collection::<BsonPatient>("patients");
    let practitioner_collection = db.collection::<BsonPractitioner>("practitioners");

    MongoCollections {
        patient: patient_collection, // These fields of type Collection<T> will be passed to AppState struct which will be passed to .with_state() method who requires State<T> type. Here State<AppState>
        practitioner: practitioner_collection
    }
}
