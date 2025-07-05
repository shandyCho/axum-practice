use serde::{Deserialize, Serialize};   
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Employee {
    id: u32,
    name: &'static str,
    department: &'static str,
    join_date: &'static str,
    is_working: bool,
    is_late: bool,
    total_vacation_date: i32,
    vacation_info: Vec<VacationInfo>
}

#[derive(Serialize, Deserialize)]
pub struct VacationInfo {
    vacation_time_type: &'static str,
    vacation_range_type: &'static str,
    vacation_date: &'static str,
    vacation_user_name: &'static str
}

pub static FIRST_EMPLOYEE: Mutex<Employee> = Mutex::new(Employee {
    id: 1,
    name: "박상길",
    department: "인사팀",
    join_date: "2022-04-19",
    is_working: false,
    is_late: false,
    total_vacation_date: 15,
    vacation_info: vec![]
});
pub static SECOND_EMPLOYEE: Mutex<Employee> = Mutex::new(Employee {
    id: 2,
    name: "정진호",
    department: "기획팀",
    join_date: "2012-02-20",
    is_working: false,
    is_late: false,
    total_vacation_date: 19,
    vacation_info: vec![]
});
pub static THIRD_EMPLOYEE: Mutex<Employee> = Mutex::new(Employee {
    id:3,
    name: "연민수",
    department: "해외영업팀",
    join_date: "2010-01-11",
    is_working: false,
    is_late: false,
    total_vacation_date: 19,
    vacation_info: vec![]
});