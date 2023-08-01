use mongodb::bson::{self, Document};

pub trait MongoRequest {
    type OptionsType;

    fn coll(&self) -> &str;

    fn opts(&self) -> Self::OptionsType;   
}

pub trait FilterQuery {
    fn filter(&self) -> Option<Result<Document, bson::ser::Error>>;
}

pub trait DocumentPayload {
    type PayloadType;

    fn payload(&self) -> Result<Self::PayloadType, bson::ser::Error>;
}