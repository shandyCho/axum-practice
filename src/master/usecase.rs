use serde_json::{json, Value};

pub async fn get_initial_data() -> Vec<Value> {
    let mut initial_data_vector = Vec::new();
    let first_employee = json!({
      "id": 1,
      "name": "박상길",
      "department": "인사팀",
      "joinDate": "2022-04-19",
      "isWorking": false,
      "isLate": false,
      "totalVacationDate": 15,
      "vacationInfo": []
    });
    let second_employee = json!({
      "id": 2,
      "name": "정진호",
      "department": "기획팀",
      "joinDate": "2012-02-20",
      "isWorking": false,
      "isLate": false,
      "totalVacationDate": 19,
      "vacationInfo": []
    });
    let third_employee = json!({
      "id":3,
      "name": "연민수",
      "department": "해외영업팀",
      "joinDate": "2010-01-11",
      "isWorking": false,
      "isLate": false,
      "totalVacationDate": 19,
      "vacationInfo": []
    });

    initial_data_vector.push(first_employee);
    initial_data_vector.push(second_employee);
    initial_data_vector.push(third_employee);
    initial_data_vector
}