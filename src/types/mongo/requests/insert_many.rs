use serde::{Deserialize, Serialize};
use serde_json::Value;
use mongodb::{options::{InsertManyOptions, WriteConcern}, bson::{self, Document}};

use crate::{types::mongo::traits::requests::{MongoRequest, DocumentPayload}, utils::mongo::parse_docs};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertManyRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    documents: Vec<Value>,
    bypass_document_validation: Option<bool>,
    write_concern: Option<WriteConcern>,
    ordered: Option<bool>
}

impl MongoRequest for InsertManyRequest {
    type OptionsType = InsertManyOptions;

    fn coll(&self) -> &str {
        &self.collection
    }

    fn opts(&self) -> Self::OptionsType {
        let mut insert_many_opts = InsertManyOptions::default();

        if let Some(bypass_document_validation) = self.bypass_document_validation {
            insert_many_opts.bypass_document_validation = Some(bypass_document_validation.clone());
        }

        if let Some(write_concern) = &self.write_concern {
            insert_many_opts.write_concern = Some(write_concern.clone());
        }

        if let Some(ordered) = self.ordered {
            insert_many_opts.ordered = Some(ordered.clone());
        }

        insert_many_opts
    }
}

impl DocumentPayload for InsertManyRequest {
    type PayloadType = Vec<Document>;

    fn payload(&self) -> Result<Self::PayloadType, bson::ser::Error> {
        parse_docs(&self.documents)
    }
}