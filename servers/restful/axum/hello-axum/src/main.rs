use axum::{self, Router, routing::get, response::{Html, IntoResponse}, extract::{Query, Path}};
use tokio;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn routes() -> Router {
    Router::new()
    .route("/hello", get(hello))
    .route("/hello2", get(hello2))
    .route("/hello3/:name", get(hello3))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn hello() -> impl IntoResponse {
    Html(format!("Hello axum!"))
}

async fn hello2(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or_default();
    Html(format!("Hello {name}!"))
}

async fn hello3(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello {name}!"))
}