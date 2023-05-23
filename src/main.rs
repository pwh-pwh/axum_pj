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
use axum::{middleware, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

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
    res
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
