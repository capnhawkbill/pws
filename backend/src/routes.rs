//! All the routes that form the api

use anyhow::Result;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::UserDataBase;
use crate::login;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ApiKey {
    apikey: String,
}

/// Just sends everything to the login function
#[post("/login", format = "json", data = "<credentials>")]
pub fn login(conn: UserDataBase, credentials: Json<Credentials>) -> Result<Json<ApiKey>> {
    let apikey = match login::login(
        &conn,
        (*credentials.username).to_string(),
        (*credentials.password).to_string(),
    ) {
        Ok(v) => v,
        // If the username or password is incorrect return that otherwise error
        Err(e) => match e {
            _ => todo![],
        },
    };
    Ok(Json(ApiKey { apikey: apikey }))
}

/// Just sends everything to the signup function
#[post("/signup", format = "json", data = "<credentials>")]
pub fn signup(conn: UserDataBase, credentials: Json<Credentials>) -> Result<Json<ApiKey>> {
    let apikey = match login::signup(
        &conn,
        (*credentials.username).to_string(),
        (*credentials.password).to_string(),
    ) {
        Ok(v) => v,
        // If the username or password is incorrect return that otherwise error
        Err(e) => match e {
            _ => todo![],
        },
    };
    Ok(Json(ApiKey { apikey: apikey }))
}
