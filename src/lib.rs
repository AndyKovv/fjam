extern crate bcrypt;
extern crate dotenv;
extern crate gotham;
extern crate borrow_bag;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate futures;
extern crate mime;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate fjam_derive;
extern crate convert;
extern crate regex;
extern crate rmp_serde as rmps;

#[macro_use]
extern crate log;

#[macro_use]
extern crate gotham_derive;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

pub mod postgres_connection;
pub mod test_utils;
pub mod user;
pub mod schema;
pub mod config;
pub mod middlewares;
pub mod common;
pub mod routes;
pub mod auth_middleware;