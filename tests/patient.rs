use axum::http::StatusCode;
use dotenvy::dotenv;
use mongodb::bson::doc;
use reqwest;
use sample_fhir_server::configuration::{init_db, MongoCollections};
use sample_fhir_server::routes::app_routes;
use sample_fhir_server::state::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

async fn test_app() -> SocketAddr {
    dotenv().ok();

    let collections: MongoCollections = init_db().await;

    let app_state = AppState {
        patients: Arc::new(collections.patient),
        practitioners: Arc::new(collections.practitioner),
    };

    let app = app_routes().await.with_state(app_state).layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = axum::serve(listener, app);
    tokio::spawn(async move {
        server.await.unwrap();
    }); // Start the server as a background task

    return addr;
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

    // No need to shutdown the server manually. tokio::spawn() has detached the server which handles
    // the server abortion for us.
}

#[tokio::test] // This test will cover the get patient with id functionality, update and delete.
async fn test_create_update_delete_patient() {
    let test_app_socket = test_app().await;
    let client = reqwest::Client::new();

    let test_patient = r#"
        {
  "resourceType" : "Patient",
  "id" : "example",
  "identifier" : [{
    "use" : "usual",
    "type" : {
      "coding" : [{
        "system" : "http://terminology.hl7.org/CodeSystem/v2-0203",
        "code" : "MR"
      }]
    },
    "system" : "urn:oid:1.2.36.146.595.217.0.1",
    "value" : "12345",
    "period" : {
      "start" : "2001-05-06"
    },
    "assigner" : {
      "display" : "Acme Healthcare"
    }
  }],
  "active" : true,
  "name" : [{
    "use" : "official",
    "family" : "Chalmers",
    "given" : ["Peter",
    "James"]
  },
  {
    "use" : "usual",
    "given" : ["Jim"]
  },
  {
    "use" : "maiden",
    "family" : "Windsor",
    "given" : ["Peter",
    "James"],
    "period" : {
      "end" : "2002"
    }
  }],
  "telecom" : [{
    "use" : "home"
  },
  {
    "system" : "phone",
    "value" : "(03) 5555 6473",
    "use" : "work",
    "rank" : 1
  },
  {
    "system" : "phone",
    "value" : "(03) 3410 5613",
    "use" : "mobile",
    "rank" : 2
  },
  {
    "system" : "phone",
    "value" : "(03) 5555 8834",
    "use" : "old",
    "period" : {
      "end" : "2014"
    }
  }],
  "gender" : "male",
  "birthDate" : "1974-12-25",
  "_birthDate" : {
    "extension" : [{
      "url" : "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
      "valueDateTime" : "1974-12-25T14:35:45-05:00"
    }]
  },
  "deceasedBoolean" : false,
  "address" : [{
    "use" : "home",
    "type" : "both",
    "text" : "534 Erewhon St PeasantVille, Rainbow, Vic  3999",
    "line" : ["534 Erewhon St"],
    "city" : "PleasantVille",
    "district" : "Rainbow",
    "state" : "Vic",
    "postalCode" : "3999",
    "period" : {
      "start" : "1974-12-25"
    }
  }],
  "contact" : [{
    "relationship" : [{
      "coding" : [{
        "system" : "http://terminology.hl7.org/CodeSystem/v2-0131",
        "code" : "N"
      }]
    }],
    "name" : {
      "family" : "du Marché",
      "_family" : {
        "extension" : [{
          "url" : "http://hl7.org/fhir/StructureDefinition/humanname-own-prefix",
          "valueString" : "VV"
        }]
      },
      "given" : ["Bénédicte"]
    },
    "telecom" : [{
      "system" : "phone",
      "value" : "+33 (237) 998327"
    }],
    "address" : {
      "use" : "home",
      "type" : "both",
      "line" : ["534 Erewhon St"],
      "city" : "PleasantVille",
      "district" : "Rainbow",
      "state" : "Vic",
      "postalCode" : "3999",
      "period" : {
        "start" : "1974-12-25"
      }
    },
    "gender" : "female",
    "period" : {
      "start" : "2012"
    }
  }],
  "managingOrganization" : {
    "reference" : "Organization/1"
  }
}"#;

    let response = client
        .post(format!("http://{}/fhir/Patient", &test_app_socket))
        .header("Content-Type", "application/fhir+json")
        .body(test_patient)
        .send()
        .await
        .unwrap();

    // Assert that the response status is 201 Created
    assert_eq!(response.status(), StatusCode::CREATED);

    let collections: MongoCollections = init_db().await;
    let patient_collection = &collections.patient;

    let filter = doc! { "id": "example" };
    let found_patient = patient_collection.find_one(filter).await.unwrap();

    // Assert that the patient was found in the database and queried successfully.
    assert!(found_patient.is_some());
    let patient = found_patient.unwrap();
    let id = patient.patient.0.id;

    // Assert the patient's ID is correct.
    assert_eq!(id, Some("example".to_string()));

    // This will update the created patient above (changed name.use, name.family).
    let updated_test_patient = r#"
        {
  "resourceType" : "Patient",
  "id" : "example",
  "identifier" : [{
    "use" : "usual",
    "type" : {
      "coding" : [{
        "system" : "http://terminology.hl7.org/CodeSystem/v2-0203",
        "code" : "MR"
      }]
    },
    "system" : "urn:oid:1.2.36.146.595.217.0.1",
    "value" : "12345",
    "period" : {
      "start" : "2001-05-06"
    },
    "assigner" : {
      "display" : "Acme Healthcare"
    }
  }],
  "active" : true,
  "name" : [{
    "use" : "usual",
    "family" : "Wellwood",
    "given" : ["Peter",
    "James"]
  },
  {
    "use" : "usual",
    "given" : ["Jim"]
  },
  {
    "use" : "maiden",
    "family" : "Windsor",
    "given" : ["Peter",
    "James"],
    "period" : {
      "end" : "2002"
    }
  }],
  "telecom" : [{
    "use" : "home"
  },
  {
    "system" : "phone",
    "value" : "(03) 5555 6473",
    "use" : "work",
    "rank" : 1
  },
  {
    "system" : "phone",
    "value" : "(03) 3410 5613",
    "use" : "mobile",
    "rank" : 2
  },
  {
    "system" : "phone",
    "value" : "(03) 5555 8834",
    "use" : "old",
    "period" : {
      "end" : "2014"
    }
  }],
  "gender" : "male",
  "birthDate" : "1974-12-25",
  "_birthDate" : {
    "extension" : [{
      "url" : "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
      "valueDateTime" : "1974-12-25T14:35:45-05:00"
    }]
  },
  "deceasedBoolean" : false,
  "address" : [{
    "use" : "home",
    "type" : "both",
    "text" : "534 Erewhon St PeasantVille, Rainbow, Vic  3999",
    "line" : ["534 Erewhon St"],
    "city" : "PleasantVille",
    "district" : "Rainbow",
    "state" : "Vic",
    "postalCode" : "3999",
    "period" : {
      "start" : "1974-12-25"
    }
  }],
  "contact" : [{
    "relationship" : [{
      "coding" : [{
        "system" : "http://terminology.hl7.org/CodeSystem/v2-0131",
        "code" : "N"
      }]
    }],
    "name" : {
      "family" : "du Marché",
      "_family" : {
        "extension" : [{
          "url" : "http://hl7.org/fhir/StructureDefinition/humanname-own-prefix",
          "valueString" : "VV"
        }]
      },
      "given" : ["Bénédicte"]
    },
    "telecom" : [{
      "system" : "phone",
      "value" : "+33 (237) 998327"
    }],
    "address" : {
      "use" : "home",
      "type" : "both",
      "line" : ["534 Erewhon St"],
      "city" : "PleasantVille",
      "district" : "Rainbow",
      "state" : "Vic",
      "postalCode" : "3999",
      "period" : {
        "start" : "1974-12-25"
      }
    },
    "gender" : "female",
    "period" : {
      "start" : "2012"
    }
  }],
  "managingOrganization" : {
    "reference" : "Organization/1"
  }
}"#;

    let response = client
        .put(format!("http://{}/fhir/Patient/example", &test_app_socket))
        .header("Content-Type", "application/fhir+json")
        .body(updated_test_patient)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // This will delete the updated patient above.
    let response = client
        .delete(format!("http://{}/fhir/Patient/example", &test_app_socket))
        .send()
        .await
        .unwrap();

    // Assert that the response status is 204 NO_CONTENT
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
