use crate::AppState;
use crate::models::dto::{CreateUrlResponse, ErrorResponse};
use crate::models::{dto::CreateUrlRequest, dto::Url};
use crate::utils::validate_url;

use actix_web::{HttpResponse, Responder, get, post, web};

#[post("/")]
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

#[get("/info/{code}")]
async fn get(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let code = path.into_inner();
    let url = data.url_service.get_url(code);
    println!("url: {:?}", url);

    match url {
        Ok(url) => HttpResponse::Ok().json(url),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            message: e.to_string(),
        }),
    }
}
#[get("/{code}")]
async fn redirect(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let code = path.into_inner();
    let url_result = data.url_service.get_url(code.clone());
    match url_result {
        Ok(url) => match data.url_service.increment_uses(code.clone()) {
            Ok(_) => HttpResponse::PermanentRedirect()
                .append_header(("Location", url.get_link()))
                .finish(),
            Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
                message: e.to_string(),
            }),
        },
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            message: e.to_string(),
        }),
    }
}
