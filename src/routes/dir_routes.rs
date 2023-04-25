use actix_web::{ web, HttpResponse, Result };

use crate::handlers::explorer_render;

pub fn dir_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/files")
        .route(web::get().to(explorer_render))
    );
}