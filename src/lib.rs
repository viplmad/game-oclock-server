#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

pub mod auth;
pub mod entities;
pub mod extractors;
pub mod mappers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod services;

pub mod convert_utils;
pub mod date_utils;
pub mod errors;
pub mod migrations;
pub mod openapi;
pub mod providers;
pub mod string_utils;
pub mod uuid_utils;
pub mod vec_utils;
