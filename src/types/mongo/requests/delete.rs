use serde::{Deserialize, Serialize};
use serde_json::Value;
use mongodb::{options::{DeleteOptions, WriteConcern}, bson::{self, Document}};

use crate::types::mongo::traits::requests::{FilterQuery, MongoRequest};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    filter: Option<Value>,
    write_concern: Option<WriteConcern>
}

impl MongoRequest for DeleteRequest {
    type OptionsType = DeleteOptions;

    fn coll(&self) -> &str {
        &self.collection
    }

    fn opts(&self) -> Self::OptionsType {
        let mut del_options = DeleteOptions::default();

        if let Some(write_concern) = &self.write_concern {
            del_options.write_concern = Some(write_concern.clone());
        }

        del_options
    }
}

impl FilterQuery for DeleteRequest {
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