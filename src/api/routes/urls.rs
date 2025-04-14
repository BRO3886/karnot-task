use crate::AppState;
use crate::models::dto::{CreateUrlResponse, ErrorResponse};
use crate::models::{dto::CreateUrlRequest, dto::Url};
use crate::utils::validate_url;

use actix_web::{HttpResponse, Responder, post, web};

#[post("/urls")]
async fn create(body: web::Json<CreateUrlRequest>, data: web::Data<AppState>) -> impl Responder {
    if !validate_url(&body.link) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "invalid url".to_string(),
        });
    }

    let url = Url::new(body.link.clone());

    let id = data.url_service.create_url(url);

    match id {
        Ok(id) => HttpResponse::Ok().json(CreateUrlResponse { id }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            message: e.to_string(),
        }),
    }
}
