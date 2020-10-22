use crate::login::permission::Permission;
use anyhow::Result;
use diesel::prelude::*;

mod models;
mod schema;

/// Struct for database pooling
#[database("sqlite_users")]
pub struct UserDataBase(diesel::SqliteConnection);

pub fn create_user(
    conn: &diesel::SqliteConnection,
    username: &str,
    password: &str,
    apikey: &str,
    permission: Permission,
) -> Result<()> {
    use schema::users;

    let user = models::NewUser {
        username,
        password,
        apikey,
        permission: permission.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)?;
    Ok(())
}

pub fn get_user(conn: &diesel::SqliteConnection, name: &str) -> Result<Vec<models::User>> {
    use schema::users::dsl::*;

    let results = users.filter(username.eq(name)).load::<models::User>(conn)?;
    Ok(results)
}
