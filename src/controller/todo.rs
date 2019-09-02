use actix_web::{HttpResponse, Responder};

#[get("/todo")]
pub fn list() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{}", vec![1, 2, 3])))
}
