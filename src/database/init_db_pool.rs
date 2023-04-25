use diesel::{mysql::MysqlConnection, r2d2::ConnectionManager};
use diesel::r2d2;

use crate::{config, Config};


pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

pub fn initialize_db_pool() -> DbPool {
    let conn_spec:Config = config();
    let manager:ConnectionManager<MysqlConnection> = r2d2::ConnectionManager::<MysqlConnection>::new(conn_spec.database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("\x1b[31;1mInvalid database URL\x1b[0m")
}