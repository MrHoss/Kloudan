use crate::{database::{init_db_pool::DbPool, schema::users::dsl::*, User}};
use actix_web::{
    error::{ErrorConflict, ErrorInternalServerError},
    web::Data,
    Result,
};
use diesel::{result::DatabaseErrorKind::UniqueViolation, result::Error};
use diesel:: RunQueryDsl;

fn is_unique_violation(err: &diesel::result::Error) -> bool {
    matches!(
        err,
        diesel::result::Error::DatabaseError(UniqueViolation, _)
    )
}
pub async fn register_user(pool: Data<DbPool>, user: &User) -> Result<User, actix_web::Error> {
    let mut conn = pool.get().unwrap();
    match diesel::insert_into(users)
        .values(user)
        .execute(&mut conn)
        .map_err(|err:Error| {
            if is_unique_violation(&err) {
                ErrorConflict("Conflict".to_string())
            } else {
                ErrorInternalServerError(err)
            }
        }) {
        Ok(_) => Ok(user.clone()),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
