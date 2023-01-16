use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
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
            routes::get_platform_games,
            routes::get_games,
            routes::post_game,
            routes::post_game_cover,
            routes::put_game,
            routes::put_game_cover,
            routes::link_game_tag,
            routes::link_game_platform,
            routes::delete_game,
            routes::delete_game_cover,
            routes::unlink_game_tag,
            routes::unlink_game_platform,
            // Game Finish
            routes::get_first_game_finish,
            routes::get_game_finishes,
            routes::get_first_finished_games,
            routes::get_last_finished_games,
            routes::post_game_finish,
            routes::delete_game_finish,
            // Game Logs
            routes::get_game_logs,
            routes::get_total_game_logs,
            routes::get_played_games,
            routes::get_first_played_games,
            routes::get_last_played_games,
            routes::post_game_log,
            routes::delete_game_log,
            // DLCs
            routes::get_dlc,
            routes::get_dlc_base_game,
            routes::get_game_dlcs,
            routes::get_platform_dlcs,
            routes::get_dlcs,
            routes::post_dlc,
            routes::post_dlc_cover,
            routes::put_dlc,
            routes::put_dlc_cover,
            routes::link_dlc_game,
            routes::link_dlc_platform,
            routes::delete_dlc,
            routes::delete_dlc_cover,
            routes::unlink_dlc_game,
            routes::unlink_dlc_platform,
            // DLC Finish
            routes::get_dlc_finishes,
            routes::get_first_dlc_finish,
            routes::get_first_finished_dlcs,
            routes::get_last_finished_dlcs,
            routes::post_dlc_finish,
            routes::delete_dlc_finish,
            // Platforms
            routes::get_platform,
            routes::get_game_platforms,
            routes::get_dlc_platforms,
            routes::get_platforms,
            routes::post_platform,
            routes::post_platform_icon,
            routes::put_platform,
            routes::put_platform_icon,
            routes::delete_platform,
            routes::delete_platform_icon,
            // Tags
            routes::get_tag,
            routes::get_game_tags,
            routes::get_tags,
            routes::post_tag,
            routes::put_tag,
            routes::delete_tag,
            // Users
            routes::get_user,
            routes::get_current_user,
            routes::get_users,
            routes::post_user,
            routes::put_user,
            routes::change_password,
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
            models::GameStatus,
            models::GameLogDTO,
            models::DLCDTO,
            models::DLCPageResult,
            models::NewDLCDTO,
            models::DLCAvailableDTO,
            models::DLCWithFinishDTO,
            models::PlatformDTO,
            models::PlatformPageResult,
            models::NewPlatformDTO,
            models::PlatformAvailableDTO,
            models::PlatformType,
            models::TagDTO,
            models::TagPageResult,
            models::NewTagDTO,
            models::UserDTO,
            models::NewUserDTO,
            models::NewPasswordDTO,
            models::PasswordChangeDTO,
            models::TokenRequest,
            models::TokenResponse,
            models::GrantType,
            models::ErrorMessage,

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
                "bearer_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
    // TODO Add allOf to OpenAPI

    // Make instance variable of ApiDoc so all worker threads get the same instance.
    ApiDoc::openapi()
}
