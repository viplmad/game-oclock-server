use utoipa::{
    openapi::security::{Flow, OAuth2, Password, Scopes, SecurityScheme},
    Modify, OpenApi,
};

use crate::{models, routes};

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            // Games
            routes::get_game,
            routes::get_tag_games,
            routes::get_location_games,
            routes::get_genre_games,
            routes::get_device_games,
            routes::get_games,
            routes::post_game,
            routes::put_game,
            routes::link_game_tag,
            routes::link_game_location,
            routes::link_game_genre,
            routes::delete_game,
            routes::unlink_game_tag,
            routes::unlink_game_location,
            routes::unlink_game_genre,
            // DLCs
            routes::get_game_dlcs,
            routes::get_dlc_base_game,
            routes::link_dlc_game,
            routes::unlink_dlc_game,
            // Game Finish
            routes::get_game_finishes,
            routes::get_first_game_finish,
            routes::get_finished_games_review,
            routes::get_first_finished_games,
            routes::get_last_finished_games,
            routes::post_game_finish,
            routes::delete_game_finish,
            // Game Logs
            routes::get_game_logs,
            routes::get_total_game_logs,
            routes::get_played_games_review,
            routes::get_first_played_games,
            routes::get_last_played_games,
            routes::post_game_log,
            routes::delete_game_log,
            // Game Links
            routes::get_game_links,
            routes::post_game_link,
            routes::delete_game_link,
            // Tags
            routes::get_tag,
            routes::get_game_tags,
            routes::get_tags,
            routes::post_tag,
            routes::put_tag,
            routes::delete_tag,
            // Locations
            routes::get_location,
            routes::get_game_locations,
            routes::get_location,
            routes::post_location,
            routes::put_location,
            routes::delete_location,
            // Genres
            routes::get_genre,
            routes::get_game_genres,
            routes::get_genres,
            routes::post_genre,
            routes::put_genre,
            routes::delete_genre,
            // Devices
            routes::get_device,
            routes::get_game_devices,
            routes::get_devices,
            routes::post_device,
            routes::put_device,
            routes::delete_device,
            // Users
            routes::get_user,
            routes::get_current_user,
            routes::get_users,
            routes::post_user,
            routes::put_user,
            routes::change_password,
            routes::promote_user,
            routes::demote_user,
            routes::delete_user,
            // Authentication
            routes::token,
            // Health check
            routes::health,
        ),
        components(schemas(
            models::GameDTO,
            models::GamePageResult,
            models::NewGameDTO,
            models::GameAvailableDTO,
            models::GameWithFinishDTO,
            models::GameWithLogDTO,
            models::GameWithLogsDTO,
            models::GamesPlayedReviewDTO,
            models::GamePlayedReviewDTO,
            models::GamesFinishedReviewDTO,
            models::GameFinishedReviewDTO,
            models::GamesStreakDTO,
            models::GameLogDTO,
            models::GameStatus,
            models::StreakDTO,
            models::LogDTO,
            models::NewLogDTO,
            models::FinishDTO,
            models::NewFinishDTO,
            models::LinkDTO,
            models::NewLinkDTO,
            models::LocationDTO,
            models::LocationPageResult,
            models::NewLocationDTO,
            models::LocationAvailableDTO,
            models::DeviceDTO,
            models::DevicePageResult,
            models::NewDeviceDTO,
            models::TagDTO,
            models::TagPageResult,
            models::NewTagDTO,
            models::UserDTO,
            models::NewUserDTO,
            models::PasswordChangeDTO,
            models::TokenRequest,
            models::TokenResponse,
            models::GrantType,
            models::ErrorMessage,
            models::DateDTO,
            models::DateTimeDTO,

            models::SearchDTO,
            models::FilterDTO,
            models::SearchValue,
            models::OperatorType,
            models::ChainOperatorType,
            models::SortDTO,
            models::OrderType,

            models::Image,
        )),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    // Add security scheme component
    struct SecurityAddon;
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // Safe unwrap: there already are components registered.
            components.add_security_scheme(
                "OAuth2",
                SecurityScheme::OAuth2(OAuth2::with_description(
                    [Flow::Password(Password::new(
                        "/auth/token",
                        Scopes::from_iter([("read", "read"), ("write", "write")]),
                    ))],
                    "OAuth2 flow",
                )),
            )
        }
    }

    // Make instance variable of ApiDoc so all worker threads get the same instance.
    ApiDoc::openapi()
}
