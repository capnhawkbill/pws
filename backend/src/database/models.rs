use diesel::{Insertable, Queryable};

use crate::login::Permission;

#[derive(QueryAble)]
pub struct User {
    username: String,
    password: String,
    apikey: String,
    permission: Permission,
}
