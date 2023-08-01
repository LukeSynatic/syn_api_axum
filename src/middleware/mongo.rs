use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, body::Body, extract::State};
use hyper;
use mongodb::{Collection, bson::Document};
use serde_json::Value;

use crate::state::state::Mongo;

pub async fn collection_mw(state: State<Mongo>, req: Request<Body>, next: Next<Body>) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();
    
    if let Ok(bytes) = hyper::body::to_bytes(body).await {
        let json: Result<Value, serde_json::Error> = serde_json::from_slice(&bytes.clone());
        
        match json {
            Ok(body) => {
                if let Some(collection) = body.get("collection") {
                    if let Some(coll_name) = collection.as_str() {
                        let collection: Collection<Document> = state.db.collection(coll_name);
                        parts.extensions.insert(collection);
                        let new_req = Request::from_parts(parts, Body::from(bytes));
                        Ok(next.run(new_req).await)
                    } else {
                        return Err(StatusCode::BAD_REQUEST);
                    }
                } else {
                    return Err(StatusCode::BAD_REQUEST);
                }
            },
            Err(_) => {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
}


// This function needs the same byte cloning treatment as collection_mw, but is not in use
pub async fn bson_mw(mut req: Request<Body>, next: Next<Body>) -> Result<Response, StatusCode> {
    if let Ok(bytes) = hyper::body::to_bytes(req.body_mut()).await {
        let b: &[u8] = &*bytes;
        let doc: Result<Document, serde_json::Error> = serde_json::from_slice(b);

        match doc {
            Ok(d) => {
                req.extensions_mut().insert(d);
                return Ok(next.run(req).await);
            },
            Err(_) => {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
}