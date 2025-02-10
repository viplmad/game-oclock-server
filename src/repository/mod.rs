mod base;
pub mod device_repository;
pub mod game_available_repository;
pub mod game_finish_repository;
pub mod game_genre_repository;
pub mod game_link_repository;
pub mod game_log_repository;
pub mod game_played_device_repository;
pub mod game_repository;
pub mod game_tag_repository;
pub mod game_with_finish_repository;
pub mod game_with_log_repository;
pub mod genre_repository;
mod location_repository;
pub mod tag_repository;
pub mod user_repository;

pub use location_repository::*;
