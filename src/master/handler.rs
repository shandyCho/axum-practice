// HTTP 요청 처리를 위한 핸들러 함수 정의

// 모듈을 실제 로직이 들어있는 코드에서 설정할 경우
// 해당 경우 동일 파일 내에서만 사용할 수 있는 것으로 보임
// master 모듈의 handler 속성 설정
pub mod handler2 {
    use axum::{http::StatusCode, response::IntoResponse, Json};
    use serde_json::json;
    // 접근 제어자 pub (public) 으로 주어야 타 파일에서 인식할 수 있다
    pub async fn start2() -> impl IntoResponse {(
        StatusCode::OK,
        Json(json!({
            "message": "hello rust axum"
        })).into_response()
    )}
}

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use crate::master::usecase;

// 접근 제어자 pub (public) 으로 주어야 타 파일에서 인식할 수 있다
pub async fn start() -> impl IntoResponse {(
    StatusCode::OK,
    Json(json!({
        "message": "hello rust axum"
    })).into_response()
)}

pub async fn initial_data() -> impl IntoResponse{
    let initial_data = usecase::get_initial_data().await;
    (
        StatusCode::OK,
        Json(json!({
            "initialData": &initial_data
        })).into_response()
    )
}

    