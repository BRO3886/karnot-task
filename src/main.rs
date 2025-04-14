use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use dotenvy::dotenv;
use karnot_task::AppState;
use karnot_task::api;
use karnot_task::database::postgres::PostgresStorage;
use karnot_task::service::UrlService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let storage = PostgresStorage::new();
    let url_service = UrlService::new(storage);

    env_logger::init();
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=debug");
        }
    }

    println!("🚀 Server started successfully on port {}", port);
    HttpServer::new(move || {
        let url_service = url_service.clone();
        App::new()
            .wrap(Logger::new("%a - %r - %s - %T"))
            .configure(api::init)
            .app_data(web::Data::new(AppState { url_service }))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
