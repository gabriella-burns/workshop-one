use axum::{
    extract::{Json, Query},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;

#[derive(Deserialize, Serialize)]
struct EchoRequest {
    message: String,
}

async fn echo(Json(payload): Json<EchoRequest>) -> impl IntoResponse {
    axum::Json(payload)
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    page: Option<u32>,
}

async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    format!("Search query: {}, Page: {}", params.q, page)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/echo", post(echo))
        .route("/search", get(search));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
