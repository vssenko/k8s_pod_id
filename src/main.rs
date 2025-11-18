use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::env;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Info {
    name: String,
    group_id: String,
    shard_id: String,
}

async fn handler() -> Json<Info> {
    let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "unknown".to_string());
    let shard_id = env::var("SHARD_ID").unwrap_or_else(|_| "unknown".to_string());

    Json(Info {
        name: "fixed_name".to_string(),
        group_id,
        shard_id,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("run: Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}
