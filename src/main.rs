use std::{env, fs::File, io::BufReader};

use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use game_collection_server::clients::cloudinary_client::{
    CloudinaryConnectOptions, CloudinaryConnection,
};
use game_collection_server::clients::image_client::ImageConnection;
use game_collection_server::{models, routes};

use actix_web::{web, App, HttpServer};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::{Config, SwaggerUi};

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_HTTP_PORT: &str = "80";
const DEFAULT_HTTPS_PORT: &str = "443";

const TLS_CERT_PATH: &str = "/certs/cert.pem";
const TLS_KEY_PATH: &str = "/certs/key.pem";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_logger();

    let host = env::var("HOST").unwrap_or_else(|_| String::from(DEFAULT_HOST));
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from(DEFAULT_HTTP_PORT))
        .parse()
        .expect("Port is not a number");

    // Database
    let db_pool = get_connection_pool()
        .await
        .expect("Could not open database connection.");

    let cloudinary_connection = get_cloudinary_connection();
    // Cloudinary

    // Encoding/Decoding
    let secret_key: String = env::var("SECRET_KEY").expect("Secret key not set.");

    let encoding_key = generate_encoding_key(&secret_key);
    let decoding_key = generate_decoding_key(&secret_key);

    // TLS
    let tls_port = env::var("TLS_PORT")
        .unwrap_or_else(|_| String::from(DEFAULT_HTTPS_PORT))
        .parse()
        .expect("TLS port is not a number");
    // TODO get paths from env
    let tls_config = load_tls_config(TLS_CERT_PATH, TLS_KEY_PATH);

    run(
        host,
        port,
        db_pool,
        cloudinary_connection,
        encoding_key,
        decoding_key,
        tls_port,
        tls_config,
    )
    .await
    .expect("Could not start server.");

    Ok(())
}

fn init_logger() {
    env_logger::init();
}

async fn get_connection_pool() -> Result<PgPool, sqlx::Error> {
    let host = env::var("DB_HOST").expect("Database host not set.");
    let port = env::var("DB_PORT")
        .expect("Database port not set.")
        .parse()
        .expect("Database port is not a number.");
    let database = env::var("DB_DATABASE").expect("Database not set.");
    let user = env::var("DB_USER").expect("Database user not set.");
    let password = env::var("DB_PASSWORD").expect("Database password not set.");

    log::info!(
        "Database connected to {}:{}@{}:{}/{}",
        user,
        password,
        host,
        port,
        database
    );

    // Manually-constructed options
    let conn = PgConnectOptions::new()
        .username(&user)
        .password(&password)
        .host(&host)
        .port(port)
        .database(&database)
        .ssl_mode(PgSslMode::Prefer);

    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(5)
        .connect_with(conn)
        .await
}

fn get_cloudinary_connection() -> Option<CloudinaryConnection> {
    let cloud_name = match env::var("CLOUDINARY_CLOUD_NAME") {
        Ok(val) => Some(val),
        Err(_) => {
            log::info!("Cloudinary cloud name not set. -> Image disabled");
            None
        }
    }?;
    let api_key = match env::var("CLOUDINARY_API_KEY") {
        Ok(val) => match val.parse() {
            Ok(int_val) => Some(int_val),
            Err(_) => {
                log::info!("Cloudinary api key is not a number. -> Image disabled");
                None
            }
        },
        Err(_) => {
            log::info!("Cloudinary api key not set. -> Image disabled");
            None
        }
    }?;
    let api_secret = match env::var("CLOUDINARY_API_SECRET") {
        Ok(val) => Some(val),
        Err(_) => {
            log::info!("Cloudinary api key not set. -> Image disabled");
            None
        }
    }?;

    log::info!(
        "Cloudinary connected to {}:{}@{}",
        api_key,
        api_secret,
        cloud_name
    );

    // Manually-constructed options
    let conn = CloudinaryConnectOptions::default()
        .cloud_name(&cloud_name)
        .api_key(api_key)
        .api_secret(&api_secret);

    Some(CloudinaryConnection::default().connect_with(conn))
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
    db_pool: PgPool,
    cloudinary_connection: Option<CloudinaryConnection>,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    tls_port: u16,
    tls_config: Option<rustls::ServerConfig>,
) -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            // Games
            routes::get_game,
            routes::get_first_game_finish,
            routes::get_game_finishes,
            routes::get_total_game_logs,
            routes::get_game_logs,
            routes::get_game_dlcs,
            routes::get_game_tags,
            routes::get_game_platforms,
            routes::get_played_games,
            routes::get_first_finished_games,
            routes::get_last_finished_games,
            routes::get_first_played_games,
            routes::get_last_played_games,
            routes::get_games,
            routes::post_game,
            routes::post_game_finish,
            routes::post_game_log,
            routes::put_game,
            routes::link_game_dlc,
            routes::link_game_tag,
            routes::link_game_platform,
            routes::delete_game,
            routes::delete_game_finish,
            routes::delete_game_log,
            routes::unlink_game_dlc,
            routes::unlink_game_tag,
            routes::unlink_game_platform,
            // DLCs
            routes::get_dlc,
            routes::get_dlc_base_game,
            routes::get_first_dlc_finish,
            routes::get_dlc_finishes,
            routes::get_dlc_platforms,
            routes::get_first_finished_dlcs,
            routes::get_last_finished_dlcs,
            routes::get_dlcs,
            routes::post_dlc,
            routes::post_dlc_finish,
            routes::put_dlc,
            routes::link_dlc_platform,
            routes::delete_dlc,
            routes::delete_dlc_finish,
            routes::unlink_dlc_platform,
            // Platforms
            routes::get_platform,
            routes::get_platform_games,
            routes::get_platform_dlcs,
            routes::get_platforms,
            routes::post_platform,
            routes::put_platform,
            routes::delete_platform,
            // Tags
            routes::get_tag,
            routes::get_tag_games,
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
            routes::is_alive,
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
    let openapi = ApiDoc::openapi();

    let mut server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(game_collection_server::auth::token_validator);

        App::new()
            .app_data(web::Data::new(db_pool.clone())) // TODO Wrap dbpool
            .app_data(web::Data::new(cloudinary_connection.clone()))
            .app_data(web::Data::new(encoding_key.clone()))
            .app_data(web::Data::new(decoding_key.clone()))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .app_data(web::Data::new(encoding_key.clone()))
                        .app_data(web::Data::new(decoding_key.clone()))
                        .wrap(auth)
                        // TODO finish and log in different scopes
                        // Games
                        .service(routes::get_game)
                        .service(routes::get_first_game_finish)
                        .service(routes::get_game_finishes)
                        .service(routes::get_total_game_logs)
                        .service(routes::get_game_logs)
                        .service(routes::get_game_dlcs)
                        .service(routes::get_game_tags)
                        .service(routes::get_game_platforms)
                        .service(routes::get_played_games)
                        .service(routes::get_first_finished_games)
                        .service(routes::get_last_finished_games)
                        .service(routes::get_first_played_games)
                        .service(routes::get_last_played_games)
                        .service(routes::get_games)
                        .service(routes::post_game)
                        .service(routes::post_game_finish)
                        .service(routes::post_game_log)
                        .service(routes::put_game)
                        .service(routes::link_game_dlc)
                        .service(routes::link_game_platform)
                        .service(routes::delete_game)
                        .service(routes::delete_game_finish)
                        .service(routes::delete_game_log)
                        .service(routes::unlink_game_dlc)
                        .service(routes::unlink_game_tag)
                        .service(routes::unlink_game_platform)
                        // DLCs
                        .service(routes::get_dlc)
                        .service(routes::get_dlc_base_game)
                        .service(routes::get_first_dlc_finish)
                        .service(routes::get_dlc_finishes)
                        .service(routes::get_dlc_platforms)
                        .service(routes::get_first_finished_dlcs)
                        .service(routes::get_last_finished_dlcs)
                        .service(routes::get_dlcs)
                        .service(routes::post_dlc)
                        .service(routes::post_dlc_finish)
                        .service(routes::put_dlc)
                        .service(routes::link_dlc_platform)
                        .service(routes::delete_dlc)
                        .service(routes::delete_dlc_finish)
                        .service(routes::unlink_dlc_platform)
                        // Platforms
                        .service(routes::get_platform)
                        .service(routes::get_platform_games)
                        .service(routes::get_platform_dlcs)
                        .service(routes::get_platforms)
                        .service(routes::post_platform)
                        .service(routes::put_platform)
                        .service(routes::delete_platform)
                        // Tags
                        .service(routes::get_tag)
                        .service(routes::get_tag_games)
                        .service(routes::get_tags)
                        .service(routes::post_tag)
                        .service(routes::put_tag)
                        .service(routes::delete_tag)
                        // Users
                        .service(routes::get_user)
                        .service(routes::get_current_user)
                        .service(routes::get_users)
                        .service(routes::post_user)
                        .service(routes::put_user)
                        .service(routes::change_password)
                        .service(routes::delete_user),
                ),
            )
            // Authentication
            .service(web::scope("/auth").service(routes::token))
            // Health check
            .service(routes::is_alive)
            // OpenAPI
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

    if let Some(config) = tls_config {
        log::info!(
            "TLS enabled -> Server listening on https://{}:{}",
            host,
            tls_port
        );
        server = server.bind_rustls((host.clone(), tls_port), config)?;
    }

    log::info!("Server listening on http://{}:{}", host, port);
    server.bind((host, port))?.run().await
}

fn load_tls_config(cert_path: &str, key_path: &str) -> Option<rustls::ServerConfig> {
    // Load TLS key/cert files
    let cert_file = match File::open(cert_path) {
        Ok(file) => Some(file),
        Err(err) => {
            log::info!(
                "cert.pem file NOT found at {} -> TLS disabled. - {}",
                cert_path,
                err.to_string()
            );
            return None;
        }
    }?;
    log::info!("cert.pem file found at {}.", cert_path);

    let key_file = match File::open(key_path) {
        Ok(file) => Some(file),
        Err(err) => {
            log::info!(
                "key.pem file NOT found at {} -> TLS disabled. -  {}",
                key_path,
                err.to_string()
            );
            return None;
        }
    }?;
    log::info!("key.pem file found at {}.", key_path);

    // Init server config builder with safe defaults
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file_reader = &mut BufReader::new(cert_file);
    let key_file_reader = &mut BufReader::new(key_file);

    // Convert files to key/cert objects
    let cert_chain = match rustls_pemfile::certs(cert_file_reader) {
        Ok(certs) => Some(certs.into_iter().map(rustls::Certificate).collect()),
        Err(err) => {
            log::warn!(
                "Cert chain could not be obtained from cert.pem. -> TLS disabled - {}",
                err.to_string()
            );
            return None;
        }
    }?;

    let mut keys: Vec<rustls::PrivateKey> =
        match rustls_pemfile::pkcs8_private_keys(key_file_reader) {
            Ok(keys) => Some(keys.into_iter().map(rustls::PrivateKey).collect()),
            Err(err) => {
                log::warn!(
                    "Private keys could not be obtained from key.pem. -> TLS disabled - {}",
                    err.to_string()
                );
                return None;
            }
        }?;

    // exit if no keys could be parsed
    if keys.is_empty() {
        log::info!("Could not locate PKCS 8 private keys. -> TLS disabled");
        return None;
    }

    config
        .with_single_cert(cert_chain, keys.remove(0))
        .map_or_else(
            |err| {
                log::info!(
                    "Error creating tls config. -> TLS disabled - {}",
                    err.to_string()
                );
                None
            },
            Some,
        )
}
