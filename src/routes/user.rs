use actix_web::{get, post, patch, delete, HttpResponse, Responder, web };

use uuid::Uuid;
use crate::models::user::{NewUser, GetUser, PatchUser, get_user, ApiResponse};
use serde::Deserialize;

#[get("/api/user/{id}")]
pub async fn get(web::Path(id): web::Path<Uuid>) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(get_user().await))
}


#[patch("/api/user/{id}")]
pub async fn patch((web::Path(id), info): (web::Path<Uuid>, web::Json<PatchUser>)) -> impl Responder {
    HttpResponse::Ok().body("user")
}


#[post("/api/user/new")]
pub async fn post() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(get_user().await))
}

#[derive(Deserialize)]
struct DeletePassword {
    password: String
}

#[delete("/api/user/{id}")]
pub async fn delete((web::Path(id), web::Query(params)): (web::Path<Uuid>, web::Query<DeletePassword>)) -> impl Responder {
    use crate::models::user::{delete_user};
    match delete_user(&id, &params.password).await {
        UserDeleted => {
            HttpResponse::Ok().body("")
        },
        UserNotFound => {
            HttpResponse::NotFound()
                .content_type("application/json")
                .body("{ \"error\": \"User id was not found\" }")
        },
        _ => {
            HttpResponse::InternalServerError().body("{ \"error\": \"Internal Server Error\" }")
        }
    }
}
