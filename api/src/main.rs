use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = init_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("fail to start up server");
}

fn init_app() -> Router {
    Router::new()
        .route("/", get(welcome_handler))
        .route("/version", get(|| async { "0.0.1" }))
}

#[derive(serde::Serialize)]
struct ResponseMessage {
    code: u8,
    message: String,
}

async fn welcome_handler() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        code: 0,
        message: String::from("Welcome to localemgmt API"),
    })
}
