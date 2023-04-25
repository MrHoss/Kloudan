use actix_session::{Session};
use actix_web::{Result, error::ErrorUnauthorized, Error, web::Data};
use diesel::{self,  QueryDsl, RunQueryDsl};
use crate::database::{init_db_pool::DbPool, schema::users::dsl::*};


pub async fn init_session(
    session: Session,
    user: (i32, String, String),
) {
    session.insert("usr_id", &user.0.to_string()).unwrap();
}

pub async fn close_session(
    pool: Data<DbPool>,
    session: Session,
) {
    session.remove("usr_id");
}

pub async fn auth_session(
    pool: Data<DbPool>,
    session: Session
)->Result<(i32,String,String, String),Error>{
    if let Some(session_usr_id) = session.get::<String>("usr_id").unwrap() {
        let mut conn = pool.get().unwrap();
        let user = users
        .select((id,name,email,role))
        .find(session_usr_id.parse::<i32>().unwrap())
        .first::<(i32,String,String, String)>(&mut conn)
        .map_err(|_| ErrorUnauthorized("invalid_credentials"))?;
        Ok(user)
    } else {
        Err(ErrorUnauthorized("Not logged in"))
    }
}
