use axum::{
    extract::{Json, Path},
    http::StatusCode, 
    response::IntoResponse, 
    body::Bytes
};
use serde::{Serialize, Deserialize};

use crate::master::entity::{FIRST_EMPLOYEE, SECOND_EMPLOYEE, THIRD_EMPLOYEE};

#[derive(Deserialize, Debug)]
pub struct UserId {
    user_id: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeInformantion {
    id: u32,
    name: String,
    department: String,
    joinDate: String,
    isWorking: bool,
    isLate: bool,
    totalVacationDate: i32,
    vacationInfo: Vec<RequestVacationInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestVacationInfo {
    vacationTimeType: String,
    vacationRangeType: String,
    vacationDate: String,
    vacationUserName: String
}

pub async fn change_working_status(Path(user_id): Path<UserId>, 
Json(payload): Json<EmployeeInformantion>) -> Json<EmployeeInformantion> {
    tracing::debug!("{:?}", payload);
    tracing::debug!("{:?}", user_id.user_id);
    if user_id.user_id == 1 {
        {
            FIRST_EMPLOYEE.lock().unwrap().print_status();
        }
        {
            let mut first_employee = FIRST_EMPLOYEE.lock().unwrap();
            first_employee.is_working = payload.isWorking;
            first_employee.is_late = payload.isLate;
        }
        {
            FIRST_EMPLOYEE.lock().unwrap().print_status();
        }
        tracing::debug!("first employee info is {:?}", FIRST_EMPLOYEE);
        
    } else if user_id.user_id == 2 {
        {
            SECOND_EMPLOYEE.lock().unwrap().print_status();
        }
        {
            let mut second_employee = SECOND_EMPLOYEE.lock().unwrap();
            second_employee.is_working = payload.isWorking;
            second_employee.is_late = payload.isLate;
        }
        {
            SECOND_EMPLOYEE.lock().unwrap().print_status();
        }
        tracing::debug!("second employee info is {:?}", SECOND_EMPLOYEE)
    } else if user_id.user_id == 3 {
        {
            THIRD_EMPLOYEE.lock().unwrap().print_status();
        }
        {
            let mut third_employee = THIRD_EMPLOYEE.lock().unwrap();
            third_employee.is_working = payload.isWorking;
            third_employee.is_late = payload.isLate;
        }
        {
            THIRD_EMPLOYEE.lock().unwrap().print_status();
        }
        tracing::debug!("third employee info is {:?}", THIRD_EMPLOYEE)
    }
    Json(payload)
}
