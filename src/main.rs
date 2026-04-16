use std::env;

use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use game_oclock_server::{
    migrations, openapi, providers::SqlxPostgresPoolBuilder, repository::*, routes, services::*,
};

use actix_web::{App, HttpServer, web};
use jsonwebtoken::{DecodingKey, EncodingKey};
use utoipa_swagger_ui::{Config, SwaggerUi};

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_HTTP_PORT: &str = "80";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_logger();

    let host = env::var("HOST").unwrap_or_else(|_| String::from(DEFAULT_HOST));
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from(DEFAULT_HTTP_PORT))
        .parse()
        .expect("Port is not a number.");

    // Encoding/Decoding
    let secret_key: String = env::var("SECRET_KEY")
        .expect("Secret key not set. Set through 'SECRET_KEY' environemnt variable.");

    let encoding_key = generate_encoding_key(&secret_key);
    let decoding_key = generate_decoding_key(&secret_key);

    run(host, port, encoding_key, decoding_key)
        .await
        .expect("Could not start server.");

    Ok(())
}

fn init_logger() {
    env_logger::init();
}

fn generate_encoding_key(key: &str) -> EncodingKey {
    EncodingKey::from_secret(key.as_ref())
}

fn generate_decoding_key(key: &str) -> DecodingKey {
    DecodingKey::from_secret(key.as_ref())
}

async fn run(
    host: String,
    port: u16,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
) -> std::io::Result<()> {
    let data_encoding_key = web::Data::new(encoding_key);
    let data_decoding_key = web::Data::new(decoding_key);

    // Repository
    let database_connection_pool = SqlxPostgresPoolBuilder::from_env()
        .await
        .expect("Could not open database connection.");
    // migrations::apply_migrations(&database_connection_pool).await;

    let igdb_client = IgdbClientPoolBuilder::from_env()
        .await
        .expect("Could not create IGDB client.");

    let user_repository = UserRepository::with_connection(database_connection_pool.clone());
    let user_service = UserService::with(user_repository);
    migrations::check_admin_user(&user_service).await;

    let device_repository = DeviceRepository::with_connection(database_connection_pool.clone());
    let media_available_repository =
        MediaAvailableRepository::with_connection(database_connection_pool.clone());
    let media_session_repository =
        MediaSessionRepository::with_connection(database_connection_pool.clone());
    let media_session_device_repository =
        MediaSessionDeviceRepository::with_connection(database_connection_pool.clone());
    let media_repository = MediaRepository::with_connection(database_connection_pool.clone());
    let media_tag_repository =
        MediaTagRepository::with_connection(database_connection_pool.clone());
    let media_with_session_repository =
        MediaWithSessionRepository::with_connection(database_connection_pool.clone());
    let location_repository = LocationRepository::with_connection(database_connection_pool.clone());
    let tag_repository = TagRepository::with_connection(database_connection_pool.clone());

    let auth_service = AuthService::with(user_service.clone());
    let device_service = DeviceService::with(device_repository);
    let external_media_service = MediaExternalService::with(igdb_client);
    let media_service = MediaService::with(external_media_service.clone(), media_repository);
    let location_service = LocationService::with(location_repository);
    let tag_service = TagService::with(tag_repository);
    let media_available_service = MediaAvailableService::with(
        media_available_repository,
        media_service.clone(),
        location_service.clone(),
    );
    let media_session_service = MediaSessionService::with(
        media_session_repository,
        media_service.clone(),
        device_service.clone(),
    );
    let media_session_device_service = MediaSessionDeviceService::with(
        media_session_device_repository,
        media_service.clone(),
        device_service.clone(),
    );
    let media_tag_service = MediaTagService::with(
        media_tag_repository,
        media_service.clone(),
        tag_service.clone(),
    );
    let media_with_session_service = MediaWithSessionService::with(media_with_session_repository);
    let media_review_service = MediaReviewService::with(
        media_session_service.clone(),
        media_with_session_service.clone(),
    );

    let data_auth_service = web::Data::new(auth_service.clone());
    let data_device_service = web::Data::new(device_service.clone());
    let data_media_service = web::Data::new(media_service.clone());
    let data_location_service = web::Data::new(location_service.clone());
    let data_tag_service = web::Data::new(tag_service.clone());
    let data_media_available_service = web::Data::new(media_available_service.clone());
    let data_media_session_service = web::Data::new(media_session_service.clone());
    let data_media_session_device_service = web::Data::new(media_session_device_service.clone());
    let data_media_tag_service = web::Data::new(media_tag_service.clone());
    let data_media_with_session_service = web::Data::new(media_with_session_service.clone());
    let data_media_review_service = web::Data::new(media_review_service.clone());

    // OpenAPI
    let openapi = openapi::get_openapi();

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(game_oclock_server::auth::token_validator);

        App::new()
            // Data injection
            .app_data(data_auth_service.clone())
            .app_data(data_device_service.clone())
            .app_data(data_media_service.clone())
            .app_data(data_location_service.clone())
            .app_data(data_tag_service.clone())
            .app_data(data_media_available_service.clone())
            .app_data(data_media_session_service.clone())
            .app_data(data_media_session_device_service.clone())
            .app_data(data_media_tag_service.clone())
            .app_data(data_media_with_session_service.clone())
            .app_data(data_media_review_service.clone())
            .app_data(data_encoding_key.clone())
            .app_data(data_decoding_key.clone())
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .wrap(auth)
                        // Medias
                        .service(routes::get_media)
                        .service(routes::get_tag_medias)
                        .service(routes::count_tag_medias)
                        .service(routes::get_location_medias)
                        .service(routes::count_location_medias)
                        .service(routes::get_device_medias)
                        .service(routes::count_device_medias)
                        .service(routes::get_medias)
                        .service(routes::count_medias)
                        .service(routes::create_media)
                        .service(routes::update_media)
                        .service(routes::link_media_tag)
                        .service(routes::link_media_location)
                        .service(routes::delete_media)
                        .service(routes::unlink_media_tag)
                        .service(routes::unlink_media_location)
                        .service(routes::link_parent_media)
                        .service(routes::unlink_parent_media)
                        .service(routes::sync_media)
                        .service(routes::search_external_medias)
                        // Media Sessions
                        .service(routes::get_media_sessions)
                        .service(routes::aggregate_media_sessions)
                        .service(routes::get_session_medias_review)
                        .service(routes::get_first_session_medias)
                        .service(routes::get_last_session_medias)
                        .service(routes::get_media_session)
                        .service(routes::create_media_session)
                        .service(routes::delete_media_session)
                        // Tags
                        .service(routes::get_tag)
                        .service(routes::get_media_tags)
                        .service(routes::count_media_tags)
                        .service(routes::get_tags)
                        .service(routes::count_tags)
                        .service(routes::create_tag)
                        .service(routes::update_tag)
                        .service(routes::delete_tag)
                        // Locations
                        .service(routes::get_location)
                        .service(routes::get_media_locations)
                        .service(routes::count_media_locations)
                        .service(routes::get_locations)
                        .service(routes::count_locations)
                        .service(routes::create_location)
                        .service(routes::update_location)
                        .service(routes::delete_location)
                        // Devices
                        .service(routes::get_device)
                        .service(routes::get_media_devices)
                        .service(routes::count_media_devices)
                        .service(routes::get_devices)
                        .service(routes::count_devices)
                        .service(routes::create_device)
                        .service(routes::update_device)
                        .service(routes::delete_device)
                        // Users
                        .service(routes::get_user)
                        .service(routes::get_current_user)
                        .service(routes::get_users)
                        .service(routes::count_users)
                        .service(routes::create_user)
                        .service(routes::update_user)
                        .service(routes::change_password)
                        .service(routes::promote_user)
                        .service(routes::demote_user)
                        .service(routes::delete_user),
                ),
            )
            // Authentication
            .service(web::scope("/auth").service(routes::token))
            // Health check
            .service(routes::health)
            // OpenAPI
            .service(web::redirect("/api-docs", "/api-docs/")) // Redirect if no slash
            .service(
                SwaggerUi::new("/api-docs/{_:.*}")
                    .url("/api-docs/public-api.json", openapi.clone())
                    .config(
                        Config::new(["/api-docs/public-api.json"])
                            .doc_expansion(r#"["none"]"#)
                            .default_models_expand_depth(0),
                    ),
            )
    });

    log::info!("Server listening on http://{}:{}", host, port);
    server.bind((host, port))?.run().await
}
