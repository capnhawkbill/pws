#![feature(proc_macro_hygiene, decl_macro)]
#[warn(missing_docs)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate anyhow;
extern crate serde;

mod database;
mod login;
mod routes;
