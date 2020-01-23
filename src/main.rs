#![feature(type_ascription)]
use actix_web::{middleware, web, App, HttpServer};

mod coffee;
mod payload;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    print!("Start litening => 127.0.0.1:8088");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/start", web::post().to(coffee::route::start_pick_post))
            .route("/pick", web::post().to(coffee::route::pick_post))
            .route("/add", web::post().to(coffee::route::add_post))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
