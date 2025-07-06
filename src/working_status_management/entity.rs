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

impl EmployeeInformantion {
    pub fn get_is_working(&self) -> bool {
        self.isWorking
    }

    pub fn get_is_late(&self) -> bool {
        self.isLate
    }
}

impl UserId {
    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }
}