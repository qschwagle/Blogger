use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;


use crate::appdata::AppData;

use crate::database::user::{create_db_user, get_db_user, delete_db_user, patch_db_user, User};
use crate::models::responses::ApiResponse;


/// new user post request
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    /// email for user to be added
    pub email: String,

    /// username of the account to be added
    pub username: String,

    /// password of the account to be added
    pub password: String,
}

/// user information to be sent to user
#[derive(Serialize)]
pub struct GetUser {
    id: Uuid,
    email: String,
    username: String,
    confirmed: bool
}

#[derive(Serialize)]
pub struct GetUserPublic {
    id: Uuid,
    email: String,
    username: String,
}

/// information being updated
#[derive(Serialize, Deserialize)]
pub struct PatchUser {
    email: Option<String>,
    username: Option<String>,
    /// password is required for email, password, and username change
    password: Option<String>,
    new_password: Option<String> 
}

pub async fn create_new_user(app_data: &AppData, nu: &NewUser) -> ApiResponse {
    match create_db_user(&app_data.pool, nu).await {
        Ok(id) => {
            ApiResponse::UserCreated(GetUser{
                id: id,
                username: nu.username.clone(),
                email: nu.email.clone(),
                confirmed: false
            })
        },
        Err(err) => {
            eprintln!("create_db_user Error: {}", err);
            ApiResponse::UserCreateFailed
        }
    }
}

pub async fn mock_get_user() -> GetUser {
    GetUser {
        id: Uuid::new_v4(),
        email: String::from("john@example.com"),
        username: String::from("johndoe"),
        confirmed: true
    }
}

pub async fn get_user(app_data: &AppData, id: &Uuid) -> ApiResponse {
    match get_db_user(&app_data.pool, id).await {
        Ok(Some(user)) => {
            ApiResponse::UserRetrieved(GetUser {
                username: user.username,
                email: user.email,
                id: id.clone(),
                confirmed: user.confirmed
            })
        },
        Ok(None) => {
            ApiResponse::UserNotFound
        }
        Err(err) => {
            eprintln!("get_db_user Error: {}", err);
            ApiResponse::UserRetrieveFailed
        }
    }
}

pub async fn patch_user(app_data: &AppData, p: &PatchUser, id: &Uuid) -> ApiResponse { 
    //username
    //email
    //new password
    match patch_db_user(&app_data.pool, id, &p.username, &p.email, &p.new_password).await {
        Ok(success) => {
            match success {
                true => {
                    ApiResponse::UserPatched
               },
                false => {
                    ApiResponse::UserNotFound
                }
            }
        }
        Err(err) => {
            eprintln!("patch_db_user Error:{}", err);
            ApiResponse::DbError
        }

    }
}

pub async fn delete_user(app_data: &AppData, id: &Uuid, password: &String) -> ApiResponse {
    match delete_db_user(&app_data.pool, id).await {
        Ok(true) => {
            ApiResponse::UserDeleted
        },
        Ok(false) => {
            ApiResponse::UserNotFound
        }
        Err(_) => {
            ApiResponse::UserNotDeleted
        }
    }
}
