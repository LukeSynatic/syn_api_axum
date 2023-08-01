use axum::{Router, Json, routing::post, http::StatusCode, middleware, Extension};
use mongodb::{Collection, bson::{Document, self}, results::{InsertOneResult, InsertManyResult, UpdateResult, DeleteResult}, options::ReplaceOptions};
use serde_json::Value;

use crate::{state::state::Mongo, middleware::{mongo::collection_mw, headers::ejson_mw}, utils::mongo::docs_as_json, types::mongo::{requests::{find::FindRequest, find_one::FindOneRequest, insert_one::InsertOneRequest, insert_many::InsertManyRequest, update::{UpdateRequest, UpdateOptionsWrapper}, delete::DeleteRequest, aggregate::AggregateRequest}, traits::requests::{FilterQuery, MongoRequest, DocumentPayload}}};

pub async fn mongo_router() -> Router {
    let state = Mongo::new().await;

    Router::new()
        .route("/find", post(find))
        .route("/findOne", post(find_one))
        .route("/insertOne", post(insert_one))
        .route("/insertMany", post(insert_many))
        .route("/updateOne", post(update_one))
        .route("/updateMany", post(update_many))
        .route("/replaceOne", post(replace_one))
        .route("/deleteOne", post(delete_one))
        .route("/deleteMany", post(delete_many))
        .route("/aggregate", post(aggregate))
        .layer(middleware::from_fn_with_state(state, collection_mw))
        .layer(middleware::from_fn(ejson_mw))
}


async fn find(db: Extension<Collection<Document>>, Json(body): Json<FindRequest>) -> Result<Json<Value>, StatusCode> {
    let filter = match body.filter() {
        Some(Ok(f)) => Some(f),
        None => None,
        _ => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    if let Ok(cursor) = db.find(filter, body.opts()).await {
        if let Ok(results) = docs_as_json(cursor).await {
            Ok(Json(results))
        } else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    } 
}

async fn find_one(db: Extension<Collection<Document>>, Json(body): Json<FindOneRequest>) -> Result<Json<Document>, StatusCode> {
    let filter = match body.filter() {
        Some(Ok(f)) => Some(f),
        None => None,
        _ => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let doc: Result<Option<Document>, mongodb::error::Error> = db.find_one(filter, body.opts()).await;
    
    match doc {
        Ok(Some(result)) => Ok(Json(result)),
        Ok(None) => Ok(Json(bson::Document::new())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn insert_one(db: Extension<Collection<Document>>, Json(body): Json<InsertOneRequest>) -> Result<Json<InsertOneResult>, StatusCode> {
    let doc = match body.payload() {
        Ok(d) => d,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST)
        }
    };

    match db.insert_one(doc, body.opts()).await {
        Ok(r) => Ok(Json(r)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
} 

async fn insert_many(db: Extension<Collection<Document>>, Json(body): Json<InsertManyRequest>) -> Result<Json<InsertManyResult>, StatusCode> {
    let docs = match body.payload() {
        Ok(d) => d,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match db.insert_many(docs, body.opts()).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn update_one(db: Extension<Collection<Document>>, Json(body): Json<UpdateRequest>) -> Result<Json<UpdateResult>, StatusCode> {
    let query = match body.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let update = match body.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        } 
    };

    match db.update_one(query, update, body.opts()).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn update_many(db: Extension<Collection<Document>>, Json(body): Json<UpdateRequest>) -> Result<Json<UpdateResult>, StatusCode> {
    let query = match body.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let update = match body.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        } 
    };

    match db.update_many(query, update, body.opts()).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn replace_one(db: Extension<Collection<Document>>, Json(body): Json<UpdateRequest>) -> Result<Json<UpdateResult>, StatusCode> {
    let query = match body.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let replacement = match body.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        } 
    };

    let opts: Option<ReplaceOptions> = Some(UpdateOptionsWrapper(body.opts()).into());

    match db.replace_one(query, replacement, opts).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn delete_one(db: Extension<Collection<Document>>, Json(body): Json<DeleteRequest>) -> Result<Json<DeleteResult>, StatusCode> {
    let query = match body.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match db.delete_one(query, body.opts()).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn delete_many(db: Extension<Collection<Document>>, Json(body): Json<DeleteRequest>) -> Result<Json<DeleteResult>, StatusCode> {
    let query = match body.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match db.delete_many(query, body.opts()).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn aggregate(db: Extension<Collection<Document>>, Json(body): Json<AggregateRequest>) -> Result<Json<Value>, StatusCode> {
    let pipeline = match body.payload() {
        Ok(p) => p,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match db.aggregate(pipeline, body.opts()).await {
        Ok(cursor) => {
            if let Ok(res) = docs_as_json(cursor).await {
                Ok(Json(res))
            } else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
