use std::convert::From;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use mongodb::{options::{UpdateOptions, WriteConcern, ReplaceOptions}, bson::{self, Document}};

use crate::{types::mongo::traits::requests::{MongoRequest, DocumentPayload, FilterQuery}, utils::mongo::{parse_docs, parse_filter}};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    filter: Option<Value>,
    document: Value,
    bypass_document_validation: Option<bool>,
    write_concern: Option<WriteConcern>,
    upsert: Option<bool>,
    array_filters: Option<Vec<Value>>,
}

impl MongoRequest for UpdateRequest {
    type OptionsType = UpdateOptions;

    fn coll(&self) -> &str {
        &self.collection
    }

    fn opts(&self) -> Self::OptionsType {
        let mut update_one_opts = UpdateOptions::default();

        if let Some(bypass_document_validation) = self.bypass_document_validation {
            update_one_opts.bypass_document_validation = Some(bypass_document_validation.clone());
        }

        if let Some(upsert) = self.upsert {
            update_one_opts.upsert = Some(upsert.clone());
        }

        if let Some(write_concern) = &self.write_concern {
            update_one_opts.write_concern = Some(write_concern.clone());
        }

        if let Some(array_filters) = &self.array_filters {
            match parse_docs(array_filters) {
                Ok(docs) => update_one_opts.array_filters = Some(docs),
                Err(_) => update_one_opts.array_filters = None
            }
        }

        update_one_opts
    }
}

impl DocumentPayload for UpdateRequest {
    type PayloadType = Document;

    fn payload(&self) -> Result<Self::PayloadType, bson::ser::Error> {
        bson::to_document(&self.document)
    }
}

impl FilterQuery for UpdateRequest {
    fn filter(&self) -> Option<Result<Document, bson::ser::Error>> {
        if let Some(json) = &self.filter {
            Some(parse_filter(json))
        } else {
            None
        }
    }
}

pub struct UpdateOptionsWrapper(pub UpdateOptions);

impl From<UpdateOptionsWrapper> for ReplaceOptions {
    fn from(wrapper: UpdateOptionsWrapper) -> Self {
        let mut replace_opts = ReplaceOptions::default();
        replace_opts.bypass_document_validation = wrapper.0.bypass_document_validation;
        replace_opts.collation = wrapper.0.collation;
        replace_opts.upsert = wrapper.0.upsert;
        replace_opts.write_concern = wrapper.0.write_concern;
        replace_opts
    }
}