//! The backend
// TODO Fix the stupid id cloning everywhere
#![feature(proc_macro_hygiene, decl_macro)]
#![warn(missing_docs)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate log;
extern crate anyhow;
extern crate base64;
extern crate chrono;
extern crate derive_builder;
extern crate rand;
#[macro_use]
extern crate serde;
extern crate serde_json;

pub mod auth;
pub mod database;
pub mod routes;
pub mod testhelp;
