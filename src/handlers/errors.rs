use actix_session::Session;
use actix_web::{ HttpResponse, ResponseError, web::Data, error::ErrorUnauthorized };
use sessions::auth_session;
use tera::{Context, Tera};
use std::fmt;

use crate::{middlewares::sessions, database::init_db_pool::DbPool};

#[derive(Debug)]
pub enum AppError {
    InternalServerError
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppError: {}", self)
    }
}
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                let tera = Tera::new("templates/**/*").unwrap();
                let mut context = Context::new();
                context.insert("title", "Erro interno do servidor");
                let rendered = tera.render("errors/internal_error.tera", &context).unwrap();

                HttpResponse::InternalServerError()
                    .content_type("text/html; charset=utf-8")
                    .body(rendered)
            }
        }
    }
}
impl From<actix_web::Error> for AppError {
    fn from(_err: actix_web::Error) -> AppError {
        AppError::InternalServerError
    }
}
impl From<std::io::Error> for AppError {
    fn from(_err: std::io::Error) -> AppError {
        AppError::InternalServerError
    }
}


pub async fn not_found(pool:Data<DbPool>,tera: Data<Tera>,session: Session) -> Result<HttpResponse, actix_web::Error> {
    match auth_session(pool,session).await{
        Ok(user)=>{
            let mut context: Context = Context::new();
            context.insert("title", &(404));
            context.insert("status", &(404));
            context.insert("message", "Not Found");
            context.insert("user",&user);
            let rendered = tera.render("errors/error.tera", &context).unwrap();
            Ok(HttpResponse::Ok().body(rendered))
        },
        Err(_)=>Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish()),
    }
}
