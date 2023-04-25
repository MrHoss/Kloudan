use crate::database::{init_db_pool::DbPool, schema::users::dsl::*};
use crate::handlers::LoginForm;
use crate::middlewares::sessions;
use crate::tools::verify_password;
use actix_session::Session;
use actix_web::{
    web::Data,
    error::{ ErrorUnauthorized}
    };
use sessions::{init_session};
use std::result::Result;
use diesel::{self, prelude::*, QueryDsl, RunQueryDsl};

pub async fn login_user(
    session:Session,
    pool: Data<DbPool>,
    credentials: &LoginForm,
) -> Result<(i32,String, String), actix_web::Error> {
    let mut conn = pool.get().unwrap();
    let user = users
        .select((id, email, password))
        .filter(email.eq(&credentials.email))
        .first::<(i32,String, String)>(&mut conn)
        .map_err(|_| ErrorUnauthorized("invalid_credentials"))?;

    let is_valid_password = verify_password(&credentials.password, &user.2);
    if !is_valid_password {
        return Err(ErrorUnauthorized("invalid_credentials"));
    }
    init_session(session,user.clone()).await;
    Ok(user)
}




