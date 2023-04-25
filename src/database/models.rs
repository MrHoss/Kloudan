use serde::{Deserialize, Serialize};
use diesel::{prelude::*};
use crate::database::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize,AsChangeset, Insertable, Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    #[primary_key]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: String,
    pub dir_id: String,
}