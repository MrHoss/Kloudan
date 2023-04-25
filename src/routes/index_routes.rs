use actix_web::{ web, HttpResponse, Result };

use crate::handlers::index_render;

pub fn index_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/")
        .route(web::get().to(index_render))
    );
}