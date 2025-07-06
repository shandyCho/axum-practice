use serde::{Deserialize, Serialize};   
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VacationInfo {
    pub vacation_time_type: &'static str,
    pub vacation_range_type: &'static str,
    pub vacation_date: &'static str,
    pub vacation_user_name: &'static str
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Employee {
    id: u32,
    name: &'static str,
    department: &'static str,
    join_date: &'static str,
    pub is_working: bool,
    pub is_late: bool,
    pub total_vacation_date: i32,
    pub vacation_info: Vec<VacationInfo>
}

impl Employee {
    pub fn change_working_status(mut self, is_working: bool, is_late: bool) -> Mutex<Employee>{
        self.is_late = is_late;
        self.is_working = is_working;
        tracing::debug!("is changed? {:?}", self);
        self.print_status();
        let result: Employee = self;
        
        Mutex::new(result)
        
    }
    // 직원의 현재 상태를 출력하는 헬퍼 함수
    pub fn print_status(&self) {
        tracing::debug!(
            "직원 {} (ID: {}) 상태: 근무중={}, 지각여부={}",
            self.name, self.id, self.is_working, self.is_late
        );
    }
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