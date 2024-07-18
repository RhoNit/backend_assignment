#[macro_use]
extern crate diesel;

pub mod orders {
    pub mod v1 {
        tonic::include_proto!("orders.v1");
    }
}

pub mod db;
pub mod error_handler;
pub mod models;
pub mod schema;
pub mod services;
