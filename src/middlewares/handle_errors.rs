
use actix_session::Session;
use actix_web::{dev, middleware::ErrorHandlerResponse, Result, HttpResponseBuilder, http::{header::ContentType, StatusCode}, web::Data, Error};
use tera::{Tera, Context};

use crate::{middlewares::auth_session, database::init_db_pool::DbPool};

pub fn handle_errors<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let status = res.status();
    let tera = Tera::new("templates/**/*").unwrap();
    let request = res.into_parts().0;
    let error_message = match status.as_u16() {
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a teapot",
        422 => "Unprocessable Entity",
        423 => "Locked",
        424 => "Failed Dependency",
        425 => "Too Early",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        506 => "Variant Also Negotiates",
        507 => "Insufficient Storage",
        508 => "Loop Detected",
        510 => "Not Extended",
        511 => "Network Authentication Required",
        _ => "Unknown Error",
    };
    let mut context = Context::new();
    context.insert("title", &status.as_u16());
    context.insert("status", &status.as_u16());
    context.insert("message", &error_message);

    println!("ERROR: Message:{}, Status:{}",&error_message,&status.as_u16());
    let rendered_template = tera.render("errors/error.tera", &context).unwrap();
    let new_response = match status.as_u16(){
        401 =>HttpResponseBuilder::new(StatusCode::FOUND).append_header(("Location", "/login")).finish(),
        409 =>HttpResponseBuilder::new(StatusCode::FOUND).append_header(("Location", "/signup")).finish(),
        _ => HttpResponseBuilder::new(status).insert_header(ContentType::html()).body(rendered_template)
    };
        

    Ok(ErrorHandlerResponse::Response(
        dev::ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}
