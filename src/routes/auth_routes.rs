use actix_web::web;
use crate::handlers::{render_login,handle_signup,render_signup, handle_login, handle_logout};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/signup")
        .route(web::post().to(handle_signup))
        .route(web::get().to(render_signup))
    );
    cfg.service(web::resource("/login")
        .route(web::post().to(handle_login))
        .route(web::get().to(render_login))
    );
    cfg.route("/logout", web::get().to(handle_logout));
}
