use crate::{
    master::entity::{
        FIRST_EMPLOYEE, 
        SECOND_EMPLOYEE, 
        THIRD_EMPLOYEE
    },
    working_status_management::{
        entity::{
            EmployeeInformantion
        }
    }
};

pub async fn change_working_status(payload: &EmployeeInformantion, user_id: &u32) {
    {
        FIRST_EMPLOYEE.lock().unwrap().print_status();
        SECOND_EMPLOYEE.lock().unwrap().print_status();
        THIRD_EMPLOYEE.lock().unwrap().print_status();
    }
    {
        let mut employee;
        if *user_id == 1 {
            employee = FIRST_EMPLOYEE.lock().unwrap();
        } else if *user_id == 2 {
            employee = SECOND_EMPLOYEE.lock().unwrap();
        } else if *user_id == 3 {
            employee = THIRD_EMPLOYEE.lock().unwrap();
        } else {
            tracing::error!("Invalid user ID: {}", user_id);
            return;
        }
        employee.is_working = payload.get_is_working();
        employee.is_late = payload.get_is_late();
    }
    {
        FIRST_EMPLOYEE.lock().unwrap().print_status();
        SECOND_EMPLOYEE.lock().unwrap().print_status();
        THIRD_EMPLOYEE.lock().unwrap().print_status();
    }
}
