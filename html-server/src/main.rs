use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/", get(root).post(root_post));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await;
    if let Ok(res) = listener {
        axum::serve(res, app).await.unwrap();
    }
}

async fn root() -> &'static str {
    println!("Root requested correctly");
    "Hello, world!"
}

async fn root_post(Json(payload): Json<Message>) -> StatusCode {
    println!("Root-post requested");
    let msg = payload.msg;
    println!("Message: {msg} ");

    StatusCode::ACCEPTED
}

#[derive(Deserialize)]
struct Message {
    pub msg: String,
}
