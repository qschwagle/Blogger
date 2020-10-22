use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub enum ApiResponse {
    UserNotFound,
    UserDeleted
}

/// represents a User account in the database
pub struct User {
    /// uuid associated with account
    id: Uuid,

    /// next unverified email
    new_email: Option<String>,

    /// email associated with account <must be unique>
    /// used for user login
    email: String,

    /// user name  <must be unique?>
    /// used for possible user login
    username: String,

    /// password hash
    passhash: String,

    /// email confirmed
    confirmed: bool,
}

/// new user post request
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    /// email for user to be added
    email: String,

    /// username of the account to be added
    username: String,

    /// password of the account to be added
    password: String,
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

pub async fn get_user() -> GetUser {
    GetUser {
        id: Uuid::new_v4(),
        email: String::from("john@example.com"),
        username: String::from("johndoe"),
        confirmed: true
    }
}

pub async fn delete_user(id: &Uuid, password: &String) -> ApiResponse {
    ApiResponse::UserDeleted
}
