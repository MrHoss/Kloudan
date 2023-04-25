use actix_web::{web, HttpResponse};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users")
        .route(web::get().to(|| HttpResponse::Ok()))
        .route(web::post().to(|| HttpResponse::Ok()))
    );
    cfg.service(web::resource("/users/{id}")
        .route(web::get().to(|| HttpResponse::Ok()))
        .route(web::put().to(|| HttpResponse::Ok()))
        .route(web::delete().to(|| HttpResponse::Ok()))
    );
    cfg.service(web::resource("/profile")
        .route(web::get().to(|| HttpResponse::Ok()))
        .route(web::post().to(|| HttpResponse::Ok()))
    );
}