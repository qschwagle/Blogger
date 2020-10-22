use actix_web::{get, post, patch, delete, HttpResponse, Responder, web};

#[get("/api/note/{id}")]
pub async fn get(web::Path(id): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("user")
}


#[patch("/api/note/{id}")]
pub async fn patch(web::Path(id): web::Path<(String)>) -> impl Responder {
    HttpResponse::Ok().body("user")
}


#[post("/api/note/new")]
pub async fn post() -> impl Responder {
    HttpResponse::Ok().body("user")
}

#[delete("/api/note/{id}")]
pub async fn delete(web::Path(id): web::Path<(String)>) -> impl Responder {
    HttpResponse::Ok().body("user")
}
