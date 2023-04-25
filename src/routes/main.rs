use actix_web::{web};

use crate::handlers::not_found;

use super::{auth_routes, user_routes, index_routes, dir_routes};

pub fn routes(cfg: &mut web::ServiceConfig) {
    auth_routes(cfg);
    user_routes(cfg);
    index_routes(cfg);
    dir_routes(cfg);
    cfg.default_service(web::get().to(not_found));
}