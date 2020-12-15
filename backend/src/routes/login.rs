#![allow(dead_code)]
use anyhow::Result;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::UserDataBase;

/// The credentials that are received as json
#[derive(Deserialize)]
pub struct Credentials {
    /// The username
    username: String,
    /// The password
    password: String,
}

/// Just sends everything to the signup function
#[post("/signup", format = "json", data = "<credentials>")]
pub fn signup(conn: UserDataBase, credentials: Json<Credentials>) -> Result<Json<ApiKey>> {
    todo![]
    //    let apikey = match login::signup(
    //        &conn,
    //        (*credentials.username).to_string(),
    //        (*credentials.password).to_string(),
    //    ) {
    //        Ok(v) => v,
    //        // If the username or password is incorrect return that otherwise error
    //        Err(e) => match e {
    //            _ => todo![],
    //        },
    //    };
    //    Ok(Json(ApiKey { apikey: apikey }))
}

//#[get("/user/info")]
//pub fn info(auth: Auth) -> Result<Json<UserInfo>> {
//    todo![]
//}
