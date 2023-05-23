use crate::{web, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    //todo use db auth login
    if payload.username != "root" || payload.pwd != "root" {
        return Err(Error::LoginFail);
    }
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    //todo set cookie
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)
}
