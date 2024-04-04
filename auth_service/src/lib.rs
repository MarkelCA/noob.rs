#![allow(dead_code, unused_variables)]

mod database;
mod auth_utils;

use database::Status;
pub use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        println!("Connected to database with username: {}", creds.username);
    }

    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
