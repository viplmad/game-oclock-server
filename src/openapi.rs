use utoipa::{
    Modify, OpenApi,
    openapi::security::{Flow, OAuth2, Password, Scopes, SecurityScheme},
};

use crate::{models, routes};

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        info(license(
            name = "GNU General Public License v3.0",
            identifier = "GPL-3.0-only"
        )),
        tags(
            (name = "Medias", description = "Medias management"),
            (name = "MediaSessions", description = "Media sessions management"),
            (name = "Tags", description = "Tags management"),
            (name = "Locations", description = "Locations management"),
            (name = "Devices", description = "Devices management"),
            (name = "Users", description = "Users management"),
            (name = "Auth", description = "Authentication"),
            (name = "Health", description = "Liveness / Readiness"),
        ),
        paths(
            // Medias
            routes::get_media,
            routes::get_tag_medias,
            routes::count_tag_medias,
            routes::get_location_medias,
            routes::count_location_medias,
            routes::get_device_medias,
            routes::count_device_medias,
            routes::get_medias,
            routes::count_medias,
            routes::create_media,
            routes::update_media,
            routes::link_media_tag,
            routes::link_media_location,
            routes::delete_media,
            routes::unlink_media_tag,
            routes::unlink_media_location,
            routes::link_parent_media,
            routes::unlink_parent_media,
            routes::sync_media,
            routes::search_external_medias,
            // Media Sessions
            routes::get_media_sessions,
            routes::aggregate_media_sessions,
            routes::get_sessions,
            routes::aggregate_sessions,
            routes::aggregate_group_sessions,
            routes::aggregate_first_sessions,
            routes::get_session_streaks,
            routes::get_first_session_medias,
            routes::get_last_session_medias,
            routes::get_media_session,
            routes::create_media_session,
            routes::delete_media_session,
            // Tags
            routes::get_tag,
            routes::get_media_tags,
            routes::count_media_tags,
            routes::get_tags,
            routes::count_tags,
            routes::create_tag,
            routes::update_tag,
            routes::delete_tag,
            // Locations
            routes::get_location,
            routes::get_media_locations,
            routes::count_media_locations,
            routes::get_locations,
            routes::count_locations,
            routes::create_location,
            routes::update_location,
            routes::delete_location,
            // Devices
            routes::get_device,
            routes::get_media_devices,
            routes::count_media_devices,
            routes::get_devices,
            routes::count_devices,
            routes::create_device,
            routes::update_device,
            routes::delete_device,
            // Users
            routes::get_user,
            routes::get_current_user,
            routes::get_users,
            routes::count_users,
            routes::create_user,
            routes::update_user,
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
            models::MediaDTO,
            models::Media2DTO,
            models::MediaPageResult,
            models::NewMediaDTO,
            models::MediaAvailableDTO,
            models::MediaTagDTO,
            models::MediaSessionDTO,
            models::MediaStatus,
            models::ExternalMediaIdDTO,
            models::AvailableDTO,
            models::TaggedDTO,
            models::SessionStreakDTO,
            models::SessionDTO,
            models::NewSessionDTO,
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
            models::TagMediaDTO,
            models::UserDTO,
            models::NewUserDTO,
            models::PasswordChangeDTO,
            models::TokenRequest,
            models::TokenResponse,
            models::GrantType,
            models::ErrorMessage,
            models::DateTimeDTO,
            // Search
            models::ListSearchDTO,
            models::AggregateSearchDTO,
            models::AggregateGroupSearchDTO,
            models::AggregateMetric,
            models::AggregateResultDTO,
            models::AggregateCountMetricDTO,
            models::AggregateSumMetricDTO,
            models::AggregateGroup,
            models::AggregateFieldGroupDTO,
            models::AggregateDateHistogramGroupDTO,
            models::FilterDTO,
            models::SingleValueFilterDTO,
            models::MultipleValuesFilterDTO,
            models::NoValueFilterDTO,
            models::ChainOperatorType,
            models::SortDTO,
            models::OrderType,
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
