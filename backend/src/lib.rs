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
extern crate base64;
extern crate derive_builder;
extern crate serde;

pub mod database;
pub mod auth;
pub mod models;
pub mod routes;
