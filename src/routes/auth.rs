use actix_web::{post, HttpResponse, web };
use serde::{Deserialize, Serialize};

use crate::models::{ responses::ApiResponse, auth::authenticate};
use crate::appdata::AppData;

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize)]
struct Error {
    error: String
}


#[post("/api/auth/{}/{}")]
pub async fn post((app_data, web::Path((username, password))): (web::Data<AppData>, web::Path<(String, String)>)) -> actix_web::Result<HttpResponse> {
    match authenticate(&app_data, &username, &password).await {
        ApiResponse::UserOrPasswordNotMatch => {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(""))
        },
        _ => {
            Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(Error { error: String::from("Internal Server Error") }))
        }
    }
}
