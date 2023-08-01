use serde::{Deserialize, Serialize};
use serde_json::Value;
use mongodb::{options::FindOneOptions, bson::{self, Document}};

use crate::{types::mongo::traits::requests::{MongoRequest, FilterQuery}, utils::mongo::parse_filter};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindOneRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    filter: Option<Value>,
    projection: Option<Value>,
}

impl MongoRequest for FindOneRequest {
    type OptionsType = FindOneOptions;

    fn coll(&self) -> &str {
        &self.collection
    }

    fn opts(&self) -> Self::OptionsType {
        let mut find_options = FindOneOptions::default();

        if let Some(projection) = &self.projection {
            if let Ok(doc) = bson::to_document(projection) {
                find_options.projection = Some(doc);
            }
        }

        find_options
    }
}

impl FilterQuery for FindOneRequest {
    fn filter(&self) -> Option<Result<Document, bson::ser::Error>> {
        if let Some(json) = &self.filter {
            Some(parse_filter(json))
        } else {
            None
        }
    }
}