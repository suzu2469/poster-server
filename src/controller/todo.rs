use actix_web::{get, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[get("/todos")]
pub fn list() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ListResponse(vec![1, 2, 3])))
}

#[post("/todos")]
pub fn create() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(CreateResponse("hoge".to_string())))
}

#[put("/todos/{id}")]
pub fn update(param: web::Path<UpdateParams>, data: web::Json<UpdateDTO>) -> Result<HttpResponse> {
    println!("{} name is {}", param.id, data.name);
    Ok(HttpResponse::Ok().json(UpdateResponse {
        message: "Update Successfully".to_string(),
    }))
}

#[derive(Serialize)]
struct ListResponse(Vec<i32>);

#[derive(Serialize)]
struct CreateResponse(String);

#[derive(Debug, Deserialize)]
struct UpdateParams {
    id: i32,
}

#[derive(Deserialize)]
struct UpdateDTO {
    name: String,
}

#[derive(Serialize)]
struct UpdateResponse {
    message: String,
}
