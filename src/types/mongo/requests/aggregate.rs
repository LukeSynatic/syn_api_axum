use mongodb::{options::{WriteConcern, ReadConcern, AggregateOptions}, bson::Document};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{types::mongo::traits::requests::{MongoRequest, DocumentPayload}, utils::mongo::parse_docs};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    pipeline: Vec<Value>,
    bypass_document_validation: Option<bool>,
    write_concern: Option<WriteConcern>,
    batch_size: Option<u32>,
    read_concern: Option<ReadConcern>
}

impl MongoRequest for AggregateRequest {
    type OptionsType = AggregateOptions;

    fn coll(&self) -> &str {
        &self.collection
    }

    fn opts(&self) -> Self::OptionsType {
        let mut aggregate_opts = AggregateOptions::default();

        if let Some(bypass_document_validation) = self.bypass_document_validation {
            aggregate_opts.bypass_document_validation = Some(bypass_document_validation.clone());
        }

        if let Some(write_concern) = &self.write_concern {
            aggregate_opts.write_concern = Some(write_concern.clone());
        }

        if let Some(batch_size) = &self.batch_size {
            aggregate_opts.batch_size = Some(batch_size.clone());
        }

        if let Some(read_concern) = &self.read_concern {
            aggregate_opts.read_concern = Some(read_concern.clone());
        }

        aggregate_opts
    }
}

impl DocumentPayload for AggregateRequest {
    type PayloadType = Vec<Document>;

    fn payload(&self) -> Result<Self::PayloadType, mongodb::bson::ser::Error> {
        parse_docs(&self.pipeline)
    }
}