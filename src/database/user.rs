use sqlx::postgres::{PgPool, PgRow};
use sqlx::prelude::*;
use uuid::Uuid;
use argon2::{self, Config};

use crate::models::user::{NewUser};

/// represents a User account in the database
#[derive(sqlx::FromRow)]
pub struct User {
    /// uuid associated with account
    pub id: Uuid,

    /// next unverified email
    pub new_email: Option<String>,

    /// email associated with account <must be unique>
    /// used for user login
    pub email: String,

    /// user name  <must be unique?>
    /// used for possible user login
    pub username: String,

    /// password hash
    pub passhash: String,

    /// email confirmed
    pub confirmed: bool,
}

pub async fn get_db_user(pool: &PgPool, id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as("SELECT * from users WHERE id = $1;")
        .bind(id)
        .fetch_optional(pool).await
}


pub async fn create_db_user(pool: &PgPool,new_user: &NewUser) -> Result<Uuid, String> {
    let mut salt: [u8; 16] = [0; 16];
    for i in 0..16 {
        salt[i] = rand::random::<u8>();
    }

    match sqlx::query("INSERT INTO users(email, username, passhash) VALUES($1,$2,$3) RETURNING id;")
        .bind(&new_user.email)
        .bind(&new_user.username)
        .bind(argon2::hash_encoded(new_user.password.as_bytes(), &salt, &Config::default()).unwrap())
        .fetch(pool).next().await {
            Ok(Some(val)) => {
                Ok(val.get("id"))
            },
            Ok(None) => {
                Err(String::from("Expected id, returned none"))
            },
            Err(err) => {
                Err(String::from(format!("{}", err)))
            }
        }
}

pub async fn delete_db_user(pool: &PgPool, id: &Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query("DELETE from users WHERE id = $1;")
        .bind(id)
        .execute(pool).await.map(|x| x > 0) 
}

pub async fn patch_db_user(
    pool: &PgPool, 
    id: &Uuid, 
    username: &Option<String>, 
    email: &Option<String>, 
    new_password: &Option<String>) -> Result<bool, sqlx::Error> {

    let mut salt: [u8; 16] = [0; 16];
    for i in 0..16 {
        salt[i] = rand::random::<u8>();
    }

    let user_res = get_db_user(pool, id).await?;

    match user_res {
        Some(mut user) => {
            if let Some(name) = username {
                user.username = name.to_string();
            }
            user.new_email = email.clone().map(|x| x.to_string());
            if let Some(pass) = new_password {
                user.passhash = argon2::hash_encoded(pass.as_bytes(), &salt, &Config::default()).unwrap();
            }
            sqlx::query("UPDATE users SET new_email = $1, passhash = $2, username = $3 WHERE id = $4;")
                .bind(user.new_email)
                .bind(user.passhash)
                .bind(user.username)
                .bind(id)
                .execute(pool).await.map(|x| x > 0) 
        },
        None => {
            Ok(false)
        }
    }
}
