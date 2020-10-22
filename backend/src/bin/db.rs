//! A cli interface for the database

extern crate backend;
extern crate diesel;
extern crate dotenv;
extern crate structopt;

use backend::database;
use backend::permission::Permission;
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// View all the users
    #[structopt(name = "view")]
    View,

    /// Delete a user
    #[structopt(name = "delete")]
    Delete { username: String },

    /// Update a field of a user
    #[structopt(name = "update")]
    Update {
        username: String,

        /// The field to change with update
        #[structopt(short, long)]
        field: String,

        /// The value to change to with update
        #[structopt(short, long)]
        value: String,
    },

    /// Insert a user
    #[structopt(name = "insert")]
    Insert {
        username: String,
        password: String,
        apikey: String,
        permission: String,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Db tool")]
struct Opt {
    /// The commands that this tool can use
    #[structopt(subcommand)]
    subcommand: SubCommand,
}

fn main() {
    let opt = Opt::from_args();

    let conn = establish_connection();
    match opt.subcommand {
        SubCommand::View => view(&conn),
        SubCommand::Delete { username } => delete(&conn, username),
        SubCommand::Insert {
            username,
            password,
            apikey,
            permission,
        } => insert(
            &conn,
            username,
            password,
            apikey,
            Permission::from_str(&permission).expect("invalid permission"),
        ),
        SubCommand::Update {
            username,
            field,
            value,
        } => update(&conn, username, field, value),
    }
}

fn view(conn: &SqliteConnection) {
    for user in database::get_users(&conn).unwrap() {
        println!(
            "name: {}\npassword: {}\napikey: {}\npermission: {}\n",
            user.username, user.password, user.apikey, user.permission
        );
    }
}

fn update(conn: &SqliteConnection, username: String, field: String, value: String) {
    database::update_user(&conn, &username, &field, &value).unwrap();
    println!("Updated field {} to {}", field, value);
}

fn delete(conn: &SqliteConnection, username: String) {
    database::delete_user(&conn, &username).unwrap();
    println!("Deleted {}", username);
}

fn insert(
    conn: &SqliteConnection,
    username: String,
    password: String,
    apikey: String,
    permission: Permission,
) {
    database::create_user(&conn, &username, &password, &apikey, permission).unwrap();
    println!("Created {}", username);
}

fn establish_connection() -> diesel::SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
