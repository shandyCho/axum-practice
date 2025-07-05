// main.rs는 어플리케이션의 진입점이며 라우터 설정등을 진행할 수 있다

pub mod master;
pub mod config;

use std::time::Duration;

use axum::{
    Json, 
    Router,
    body::{Bytes, Body},
    http::{ Request, Response },
    routing::{get, post}
};

use serde::{Serialize};
use tower_http::trace::{self, TraceLayer};
use tower::ServiceBuilder;
use tracing::Span;
use config::cors_config::cors_setting;



#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    // 서버 IP 및 포트 정의
    let addr = "0.0.0.0:3000";
    tracing_subscriber::fmt::init();
    // 미들웨어 서비스 정의
    let service = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
            .on_request(|request: &Request<Body>, _span: &Span| {
                tracing::debug!("started {} {}", request.method(), request.uri().path())
            })
            .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
                tracing::debug!("response generated in {:?}", latency)
            })
        )
        .layer(cors_setting::cors_setting());
    
    // 라우터 정의
    let router = Router::new()
    .route("/", get(|| async {"index!"}))
    .route("/api/v1/users", get(get_users))
    .route("/api/v1/init", get(initial_data))
    .layer(
        ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_setting::cors_setting())
        
    );


    // 기존 한국 웹 예제에서 알려준 방법이나 axum 0.7.9 이상부터는 Server 가 없어서 실행 못한다
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    // .serve(app.into_make_service())
    // .await
    // .unwrap()
    // 서버 TCP 포트 리스닝을 통한 서버 구동
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, router).await.unwrap();
}

async fn get_users() -> Json<Message>{
    Json(Message { message: String::from("Hello, Axum") })
}
