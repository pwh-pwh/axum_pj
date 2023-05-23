mod ctx;
mod error;
mod model;
mod web;

pub use self::error::{Error, Result};
use crate::model::ModelController;
use axum::extract::{Path, Query};
use axum::middleware::map_response;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service, Route};
use axum::{middleware, Json, Router};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;
    let routes_apis = web::routes_ticket::routes(mc.clone())
        .layer(middleware::from_fn(web::mw_auth::mw_request_auth));
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("server bind {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("main_response_mapper");
    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());
    let err_resp = client_status_error
        .as_ref()
        .map(|(status_code, client_err)| {
            let client_err_body = json!({
                "error": {
                    "type": client_err.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });
            (*status_code, Json(client_err_body)).into_response()
        });
    println!("server log line {uuid} - Error: {service_error:?}");
    err_resp.unwrap_or(res)
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handle_hello1))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handle_hello1(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong>{name}</strong>"))
}
