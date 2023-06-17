use crate::{Error, Result};
use axum::routing::post;
use axum::{Json, Route};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new.route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
      "result": {
        "success": true
      }
    }));
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
