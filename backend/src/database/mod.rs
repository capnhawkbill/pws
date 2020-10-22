//! All the database handling helper functions in root
//! and the models en diesel schema in the modules
use crate::permission::Permission;
use anyhow::{anyhow, Result};
use diesel::prelude::*;

pub mod models;
pub mod schema;

/// Struct for database pooling
#[database("sqlite_users")]
pub struct UserDataBase(diesel::SqliteConnection);

/// Inserts user in the database
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
        permission: permission.to_string().to_lowercase(),
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)?;
    Ok(())
}

/// Gets a user from the database using the username
pub fn get_user(conn: &diesel::SqliteConnection, name: &str) -> Result<Vec<models::User>> {
    use schema::users::dsl::*;

    let results = users.filter(username.eq(name)).load::<models::User>(conn)?;
    Ok(results)
}

/// Get all the users from the database
pub fn get_users(conn: &diesel::SqliteConnection) -> Result<Vec<models::User>> {
    use schema::users::dsl::*;
    let results = users.load::<models::User>(conn)?;
    Ok(results)
}

/// Gets a user from the database using the apikey
pub fn get_user_by_key(conn: &diesel::SqliteConnection, key: &str) -> Result<Vec<models::User>> {
    use schema::users::dsl::*;

    let results = users.filter(apikey.eq(key)).load::<models::User>(conn)?;
    Ok(results)
}

/// Delete a user from the database using the username
pub fn delete_user(conn: &diesel::SqliteConnection, name: &str) -> Result<()> {
    use schema::users::dsl::*;

    diesel::delete(users.filter(username.eq(name))).execute(conn)?;
    Ok(())
}

/// Change a field of a user
pub fn update_user(
    conn: &diesel::SqliteConnection,
    name: &str,
    field: &str,
    value: &str,
) -> Result<()> {
    use schema::users::dsl::*;

    let user = diesel::update(users.filter(username.eq(name)));
    match field {
        "username" => user.set(username.eq(value)).execute(conn)?,
        "password" => user.set(password.eq(value)).execute(conn)?,
        "apikey" => user.set(apikey.eq(value)).execute(conn)?,
        "permission" => user.set(permission.eq(value)).execute(conn)?,
        _ => return Err(anyhow!("This field doesn't exist")),
    };
    Ok(())
}
