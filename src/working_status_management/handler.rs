use axum::{
    extract::{Json, Path},
    http::StatusCode, 
    response::IntoResponse, 
    body::Bytes
};


use crate::{ 
    working_status_management::{
        entity::{
            EmployeeInformantion, 
            UserId
        },
        usecase
    }
};



pub async fn working_status_change_handler(Path(user_id): Path<UserId>, 
Json(payload): Json<EmployeeInformantion>) -> Json<EmployeeInformantion> {
    tracing::debug!("{:?}", payload);
    tracing::debug!("{:?}", user_id.get_user_id());
    let id = user_id.get_user_id();
    usecase::change_working_status(&payload, &id).await;
    Json(payload)
}
