use std::{env, fs::File, io::BufReader};

use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use game_collection_server::{
    clients::cloudinary::{CloudinaryClient, CloudinaryClientBuilder},
    migrations, openapi,
    providers::ImageClientProvider,
    routes,
};

use actix_web::{web, App, HttpServer};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
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

    // Encoding/Decoding
    let secret_key: String = env::var("SECRET_KEY").expect("Secret key not set.");

    let encoding_key = generate_encoding_key(&secret_key);
    let decoding_key = generate_decoding_key(&secret_key);

    // TLS
    let tls_port = env::var("TLS_PORT")
        .unwrap_or_else(|_| String::from(DEFAULT_HTTPS_PORT))
        .parse()
        .expect("TLS port is not a number");
    let tls_config = load_tls_config();

    run(host, port, encoding_key, decoding_key, tls_port, tls_config)
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
        .map(|res| {
            log::info!(
                "Postgres database connected to {}:<redacted>@{}:{}/{}",
                user,
                // Hide password from info log
                host,
                port,
                database
            );
            res
        })
}

fn get_cloudinary_client_provider() -> Option<CloudinaryClient> {
    CloudinaryClientBuilder::try_from_env()
        .map(|client| CloudinaryClient::default().connect_with(client))
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
    tls_port: u16,
    tls_config: Option<rustls::ServerConfig>,
) -> std::io::Result<()> {
    let data_encoding_key = web::Data::new(encoding_key);
    let data_decoding_key = web::Data::new(decoding_key);

    // Repository
    let database_connection_pool = get_connection_pool()
        .await
        .expect("Could not open database connection.");
    migrations::apply_migrations(&database_connection_pool).await;

    let data_database_connection = web::Data::new(database_connection_pool);

    // Image client
    let image_client_provider =
        if let Some(cloudinary_client_provider) = get_cloudinary_client_provider() {
            ImageClientProvider::new(cloudinary_client_provider)
        } else {
            ImageClientProvider::empty()
        };
    let data_image_client = web::Data::new(image_client_provider);
    migrations::delete_old_temp_files().await;

    // OpenAPI
    let openapi = openapi::get_openapi();

    let mut server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(game_collection_server::auth::token_validator);

        App::new()
            .app_data(data_database_connection.clone())
            .app_data(data_image_client.clone())
            .app_data(data_encoding_key.clone())
            .app_data(data_decoding_key.clone())
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .wrap(auth)
                        // Games
                        .service(routes::get_game)
                        .service(routes::get_tag_games)
                        .service(routes::get_platform_games)
                        .service(routes::get_games)
                        .service(routes::post_game)
                        .service(routes::post_game_cover)
                        .service(routes::put_game)
                        .service(routes::put_game_cover)
                        .service(routes::link_game_platform)
                        .service(routes::link_game_tag)
                        .service(routes::delete_game)
                        .service(routes::delete_game_cover)
                        .service(routes::unlink_game_tag)
                        .service(routes::unlink_game_platform)
                        // Game Finish
                        .service(routes::get_game_finishes)
                        .service(routes::get_first_game_finish)
                        .service(routes::get_first_finished_games)
                        .service(routes::get_last_finished_games)
                        .service(routes::post_game_finish)
                        .service(routes::delete_game_finish)
                        // Game Logs
                        .service(routes::get_game_logs)
                        .service(routes::get_total_game_logs)
                        .service(routes::get_played_games)
                        .service(routes::get_played_games_detailed)
                        .service(routes::get_first_played_games)
                        .service(routes::get_last_played_games)
                        .service(routes::post_game_log)
                        .service(routes::delete_game_log)
                        // Game Streaks
                        .service(routes::get_game_streaks)
                        .service(routes::get_streaks)
                        // DLCs
                        .service(routes::get_dlc)
                        .service(routes::get_dlc_base_game)
                        .service(routes::get_game_dlcs)
                        .service(routes::get_platform_dlcs)
                        .service(routes::get_dlcs)
                        .service(routes::post_dlc)
                        .service(routes::post_dlc_cover)
                        .service(routes::put_dlc)
                        .service(routes::put_dlc_cover)
                        .service(routes::link_dlc_game)
                        .service(routes::link_dlc_platform)
                        .service(routes::delete_dlc)
                        .service(routes::delete_dlc_cover)
                        .service(routes::unlink_dlc_game)
                        .service(routes::unlink_dlc_platform)
                        // DLC Finish
                        .service(routes::get_dlc_finishes)
                        .service(routes::get_first_dlc_finish)
                        .service(routes::get_first_finished_dlcs)
                        .service(routes::get_last_finished_dlcs)
                        .service(routes::post_dlc_finish)
                        .service(routes::delete_dlc_finish)
                        // Platforms
                        .service(routes::get_platform)
                        .service(routes::get_game_platforms)
                        .service(routes::get_dlc_platforms)
                        .service(routes::get_platforms)
                        .service(routes::post_platform)
                        .service(routes::post_platform_icon)
                        .service(routes::put_platform)
                        .service(routes::put_platform_icon)
                        .service(routes::delete_platform)
                        .service(routes::delete_platform_icon)
                        // Tags
                        .service(routes::get_tag)
                        .service(routes::get_game_tags)
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

fn load_tls_config() -> Option<rustls::ServerConfig> {
    // TODO get paths from env
    let cert_path = TLS_CERT_PATH;
    let key_path = TLS_KEY_PATH;

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
