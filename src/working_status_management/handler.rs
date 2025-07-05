use axum::{
    extract::{Json, Path},
    http::StatusCode, 
    response::IntoResponse, 
    body::Bytes
};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct UserId {
    user_id: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeInformantion {
    id: u32,
    name: String,
    department: String,
    join_date: String,
    is_working: bool,
    is_late: bool,
    total_vacation_date: i32,
    vacation_info: Vec<RequestVacationInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestVacationInfo {
    vacation_time_type: String,
    vacation_range_type: String,
    vacation_date: String,
    vacation_user_name: String
}

pub async fn change_working_status(Path(user_id): Path<UserId>, 
Json(payload): Json<EmployeeInformantion>) -> Json<EmployeeInformantion> {
    tracing::debug!("{:?}", payload);
    tracing::debug!("{:?}", user_id);
    // StatusCode::OK
    Json(payload)
}
