//! The backend

#![feature(proc_macro_hygiene, decl_macro)]
#![warn(missing_docs)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate anyhow;
extern crate serde;
extern crate strum;
extern crate strum_macros;

mod database;
mod login;
pub mod routes;
