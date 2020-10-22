use actix_web:: { 
    get, 
    App, 
    HttpServer,
    Responder, 
    HttpResponse,
    middleware::Logger
};
use env_logger::Env;

mod routes;
use routes::index;
use routes::user;
use routes::post;
use routes::tag;


mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| 
        App::new()
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
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
