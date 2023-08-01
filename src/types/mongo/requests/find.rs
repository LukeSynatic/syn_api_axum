use serde::{Deserialize, Serialize};
use serde_json::Value;
use mongodb::{options::FindOptions, bson::{self, Document}};

use crate::types::mongo::traits::requests::{MongoRequest, FilterQuery};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    filter: Option<Value>,
    projection: Option<Value>,
    sort: Option<Value>,
    limit: Option<i64>,
    skip: Option<u64>,
}

impl MongoRequest for FindRequest {
    type OptionsType = FindOptions;

    fn coll(&self) -> &str {
        &self.collection
    }

    fn opts(&self) -> Self::OptionsType {
        let mut find_options = FindOptions::default();

        if let Some(limit) = self.limit {
            find_options.limit = Some(limit);
        }

        if let Some(skip) = self.skip {
            find_options.skip = Some(skip);
        }

        if let Some(sort) = &self.sort {
            if let Ok(doc) = bson::to_document(sort) {
                find_options.sort = Some(doc);
            }
        }

        if let Some(projection) = &self.projection {
            if let Ok(doc) = bson::to_document(projection) {
                find_options.projection = Some(doc);
            }
        }

        find_options
    }
}

impl FilterQuery for FindRequest {
    fn filter(&self) -> Option<Result<Document, bson::ser::Error>> {
        if let Some(json) = &self.filter {
            match bson::to_document(json) {
                Ok(doc) => Some(Ok(doc)),
                Err(e) => Some(Err(e.into())) 
            }
        } else {
            None
        }
    }
}