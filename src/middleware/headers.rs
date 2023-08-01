use axum::{middleware::Next, response::Response};
use hyper::{Request, StatusCode, header::CONTENT_TYPE, http::HeaderValue};

const JSON: &str = "application/json";
const EJSON: &str = "application/ejson";

pub async fn ejson_mw<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    if let Some(content_type) = req.headers().get(CONTENT_TYPE) {
        if let Ok(val) = content_type.to_str() {
            match val {
                JSON => {
                    return Ok(next.run(req).await)
                },
                EJSON => {
                    req.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_str(JSON).unwrap());
                    return Ok(next.run(req).await)
                },
                _ => {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
}