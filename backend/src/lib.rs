//! The backend

#![feature(proc_macro_hygiene, decl_macro)]
#![warn(missing_docs)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate anyhow;
extern crate base64;
extern crate derive_builder;
extern crate serde;
extern crate serde_json;

pub mod auth;
pub mod database;
// Temporarily disable this
pub mod routes;
