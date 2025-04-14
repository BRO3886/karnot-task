use actix_web::{Responder, get};

#[get("/health")]
async fn get() -> impl Responder {
    "OK"
}
