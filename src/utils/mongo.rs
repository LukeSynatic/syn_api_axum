use futures::StreamExt;
use mongodb::{bson::{Document, self}, Cursor};
use serde_json::{Value, json};

pub async fn docs_as_json(cursor: Cursor<Document>) -> Result<Value, mongodb::error::Error> {
    let mut result: Vec<Document> = vec![];

    let collected: Vec<Result<Document, mongodb::error::Error>> = cursor.collect().await;

    for doc in collected {
        match doc {
            Ok(d) => {
                result.push(d);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(json!(result))
}

pub fn parse_docs(json_list: &Vec<Value>) -> Result<Vec<Document>, bson::ser::Error> {
    if json_list.is_empty() {
        let result: Vec<Document> = vec![];
        return Ok(result);
    }

    json_list
        .clone()
        .into_iter()
        .map(|json| bson::to_document(&json))
        .collect()
}

pub fn parse_filter(json: &Value) -> Result<Document, bson::ser::Error> {
    match bson::to_document(json) {
        Ok(doc) => Ok(doc),
        Err(e) => Err(e.into()) 
    }
}