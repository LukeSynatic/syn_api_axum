use serde::{Deserialize, Serialize};
use serde_json::Value;
use mongodb::{options::{InsertOneOptions, WriteConcern}, bson::{self, Document}};

use crate::types::mongo::traits::requests::{MongoRequest, DocumentPayload};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertOneRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    document: Value,
    bypass_document_validation: Option<bool>,
    write_concern: Option<WriteConcern>,
}

impl MongoRequest for InsertOneRequest {
    type OptionsType = InsertOneOptions;

    fn coll(&self) -> &str {
        &self.collection
    }

    fn opts(&self) -> Self::OptionsType {
        let mut insert_one_opts = InsertOneOptions::default();

        if let Some(bypass_document_validation) = self.bypass_document_validation {
            insert_one_opts.bypass_document_validation = Some(bypass_document_validation.clone());
        }

        if let Some(write_concern) = &self.write_concern {
            insert_one_opts.write_concern = Some(write_concern.clone());
        }

        insert_one_opts
    }
}

impl DocumentPayload for InsertOneRequest {
    type PayloadType = Document;

    fn payload(&self) -> Result<Self::PayloadType, bson::ser::Error> {
        bson::to_document(&self.document)
    }
}