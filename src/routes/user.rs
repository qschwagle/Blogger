use actix_web::{get, post, patch, delete, HttpResponse, Responder, web };

use uuid::Uuid;
use crate::models::user::{NewUser, PatchUser, get_user, create_new_user, patch_user};
use crate::models::responses::ApiResponse;
use serde::{Deserialize, Serialize};

use crate::appdata::AppData;

#[derive(Serialize)]
struct Error {
    error: String
}

#[get("/api/user/{id}")]
pub async fn get((app_data, web::Path(id)): (web::Data<AppData>, web::Path<Uuid>)) -> actix_web::Result<HttpResponse> {
    match get_user(&app_data, &id).await {
        ApiResponse::UserRetrieved(u) => {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(u))
        },
        ApiResponse::UserNotFound => {
            Ok(HttpResponse::NotFound()
                .content_type("application/json")
                .json(Error { error: String::from("User Not Found") }))
        },
        _ => {
            Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(Error { error: String::from("Internal Server Error") }))
        }
    }
}


#[patch("/api/user/{id}")]
pub async fn patch((app_data, web::Path(id), info): (web::Data<AppData>, web::Path<Uuid>, web::Json<PatchUser>)) -> actix_web::Result<HttpResponse> {
    match patch_user(&app_data, &info, &id).await {
        ApiResponse::UserPatched => {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body("{}"))
        },
        ApiResponse::UserNotFound => {
            Ok(HttpResponse::NotFound()
                .content_type("application/json")
                .body("{ \"error\": \"User id was not found\" }"))
        },
        _ => {
            Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("{ \"error\": \"Internal Server Error\" }"))
        }
    }
}


#[post("/api/user/new")]
pub async fn post((new_user, app_data): (web::Json<NewUser>, web::Data<AppData>)) -> actix_web::Result<HttpResponse> {
    match create_new_user(&app_data, &new_user).await {
        ApiResponse::UserCreated(u) => {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(u))
        },
        _ => {
            Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("{ \"error\": \"Internal Server Error\" }"))
        }
    }
}

#[derive(Deserialize)]
pub struct DeletePassword {
    password: String
}

#[delete("/api/user/{id}")]
pub async fn delete((app_data, web::Path(id), web::Query(params)): (web::Data<AppData>, web::Path<Uuid>, web::Query<DeletePassword>)) -> impl Responder {
    use crate::models::user::{delete_user};
    match delete_user(&app_data, &id, &params.password).await {
        ApiResponse::UserDeleted => {
            HttpResponse::Ok()
                .content_type("application/json")
                .body("{}")
        },
        ApiResponse::UserNotFound => {
            HttpResponse::NotFound()
                .content_type("application/json")
                .body("{ \"error\": \"User id was not found\" }")
        },
        _ => {
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("{ \"error\": \"Internal Server Error\" }")
        }
    }
}
