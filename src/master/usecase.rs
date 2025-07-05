use serde_json::{json, Value};

use crate::master::entity::{FIRST_EMPLOYEE, SECOND_EMPLOYEE, THIRD_EMPLOYEE};


pub async fn get_initial_data() -> Vec<Value> {
    let mut initial_data_vector = Vec::new();
    let first_employee = FIRST_EMPLOYEE.lock().unwrap();
    let second_employee = SECOND_EMPLOYEE.lock().unwrap();
    let third_employee = THIRD_EMPLOYEE.lock().unwrap();
    initial_data_vector.push(json!(*first_employee));
    initial_data_vector.push(json!(*second_employee));
    initial_data_vector.push(json!(*third_employee));
    initial_data_vector
}
