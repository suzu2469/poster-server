use actix_web::{get, post, HttpResponse, Result};
use serde::Serialize;

#[get("/todos")]
pub fn list() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(ListResponse(vec![1, 2, 3])))
}

#[post("/todos")]
pub fn create() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(CreateResponse("hoge".to_string())))
}

#[derive(Serialize)]
struct ListResponse(Vec<i32>);

#[derive(Serialize)]
struct CreateResponse(String);
