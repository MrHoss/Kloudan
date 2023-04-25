use actix_session::Session;
use actix_web::{web::Data, HttpResponse, Result};
use tera::{Tera, Context};

use crate::{middlewares::auth_session, database::init_db_pool::DbPool};

pub async fn explorer_render(pool:Data<DbPool>, session: Session, tera: Data<Tera>) -> Result<HttpResponse> {
    match auth_session(pool,session).await{
        Ok(user)=>{
            let mut context: Context = Context::new();
            context.insert("title", "Arquivos");
            context.insert("user",&user);
            let rendered = tera.render("pages/explorer.tera", &context).unwrap();
            Ok(HttpResponse::Ok().body(rendered))
        },
        Err(_)=>Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish()),
    }
    
}