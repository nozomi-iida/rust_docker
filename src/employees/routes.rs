use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use crate::employees::Employee;

#[get("/employees")]
async fn find_all() -> impl Responder {
  HttpResponse::Ok().json(vec![
    Employee {
      id: 1,
      first_name : "Ola".to_string(),
      last_name: "John Ajiboye".to_string(),
      department: "Engineering".to_string(),
      salary: 4500,
      age: 23
    },
    Employee {
      id: 2,
      first_name : "James".to_string(),
      last_name: "Bond".to_string(),
      department: "Accounting".to_string(),
      salary: 3500,
      age: 43
    },
  ])
}
#[get("/employees/{id}")]
async fn find() -> impl Responder {
  HttpResponse::Ok().json(Employee {
    id: 2,
    first_name : "James".to_string(),
    last_name: "Bond".to_string(),
    department: "Accounting".to_string(),
    salary: 3500,
    age: 43
  })
}
pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(find_all);
  config.service(find);
}