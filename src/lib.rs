use database::postgres::PostgresStorage;
use service::UrlService;

pub mod api;
pub mod database;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod service;
pub mod utils;

pub struct AppState {
    pub url_service: UrlService<PostgresStorage>,
}
