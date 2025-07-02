use axum::routing::{get};
use axum::{Json, Router};
use serde::{Serialize};

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000";
    let app = Router::new()
    .route("/", get(|| async {"index!"}))
    .route("/api/v1/users", get(get_users));

    // 기존 한국 웹 예제에서 알려준 방법이나 axum 0.7.9 이상부터는 Server 가 없어서 실행 못한다
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    // .serve(app.into_make_service())
    // .await
    // .unwrap()

    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, app).await.unwrap();
}

async fn get_users() -> Json<Message>{
    Json(Message { message: String::from("Hello, Axum") })
}
