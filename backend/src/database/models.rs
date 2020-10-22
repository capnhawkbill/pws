use super::schema::*;
use crate::login::permission::Permission;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub apikey: String,
    pub permission: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub apikey: &'a str,
    pub permission: String,
}
