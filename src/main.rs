use actix_web:: { 
    get, 
    App, 
    HttpServer,
    Responder, 
    HttpResponse,
    web,
    middleware::Logger
};
use env_logger::Env;


mod appdata;
mod database;

use appdata::AppData;

use std::env;
use sqlx::postgres::PgPool;

mod routes;
use routes::index;
use routes::user;
use routes::post;
use routes::tag;

mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(err) => {
            println!("Error: {}", err);
            return Ok(())
        }
    };

    println!("DATABASE_URL:{}", db_url);
    let pool = match PgPool::builder()
        .max_size(5)
        .build(&db_url).await {
        Ok(pool) => pool,
        Err(err) => {
            println!("Error: {}", err);
            return Ok(())
        }
    };

    let data = web::Data::new(AppData { pool: pool.clone() });

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(move || 
        App::new()
        .app_data(data.clone())
        .wrap(Logger::default())
        .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
        .service(index::get)
        .service(user::get)
        .service(user::post)
        .service(user::patch)
        .service(user::delete)
        .service(post::get)
        .service(post::post)
        .service(post::patch)
        .service(post::delete)
        )
        .bind("167.71.169.97:8080")?
        .run()
        .await
}
