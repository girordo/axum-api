#![allow(unused)]

use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));
    println!("Hello, world!");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!!");

    Html("Hello <strong>{name}</strong>")
}
