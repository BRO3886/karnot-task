use crate::models::dto::{CreateUrlResponse, ErrorResponse};
use crate::models::{dto::CreateUrlRequest, dto::Url};
use crate::utils::validate_url;
use actix_web::{HttpResponse, Responder, post, web};

#[post("/urls")]
async fn create(body: web::Json<CreateUrlRequest>) -> impl Responder {
    if !validate_url(&body.link) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "invalid url".to_string(),
        });
    }
    let url = Url::new(body.link.clone());
    HttpResponse::Ok().json(CreateUrlResponse { id: url.get_id() })
}
