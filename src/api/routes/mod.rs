use actix_web::web;

mod health;
mod urls;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health::get);
    cfg.service(urls::create);
    cfg.service(urls::get);
    cfg.service(urls::redirect);
}
