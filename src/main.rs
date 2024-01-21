#![allow(unused)]

pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::{middleware, Router, ServiceExt};
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::Response;
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse};
use tokio::net::TcpListener;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use crate::model::ModelController;
use crate::web::mw_auth::mw_require_auth;
use crate::web::routes_tickets::AppState;

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc)
        .route_layer(middleware::from_fn(mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:       --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("-> LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion:    --- Start Server

    Ok(())
}

async fn main_response_mapper(res: Response<Body>) -> Response<Body> {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}


// region:    --- Routes Static
fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get_service(ServeDir::new("./")))
}
// endregion: --- Routes Static


// region:    --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}
// endregion: --- Routes Hello

// region:    --- Handler Hello

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// e.g., `/hello?name=Shane`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

//e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 {name:?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}</strong"))

}
// endregion: --- Handler Hello
