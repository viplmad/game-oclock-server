use std::env;

use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
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
use utoipa_swagger_ui::SwaggerUi;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_logger();

    let host: String = env::var("HOST").unwrap_or_else(|_| String::from(DEFAULT_HOST));
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from(DEFAULT_PORT))
        .parse()
        .expect("Port is not a number");

    let database_user: String = env::var("DB_USER").expect("Database user not set");
    let database_password: String = env::var("DB_PASSWORD").expect("Database password not set");
    let database_host: String = env::var("DB_HOST").expect("Database host not set");
    let database_port: u16 = env::var("DB_PORT")
        .expect("Database port not set")
        .parse()
        .expect("Port is not a number");
    let database_database: String = env::var("DB_DATABASE").expect("Database not set");

    let db_pool: PgPool = get_connection_pool(
        database_user,
        database_password,
        database_host,
        database_port,
        database_database,
    )
    .await
    .expect("Could not open database connection");

    let secret_key: String = env::var("SECRET_KEY").expect("Secret key not set");

    let encoding_key = generate_encoding_key(&secret_key);
    let decoding_key = generate_decoding_key(&secret_key);

    run(host, port, db_pool, encoding_key, decoding_key)
        .await
        .expect("Could not start server");

    Ok(())
}

fn init_logger() {
    env_logger::init();
}

async fn get_connection_pool(
    user: String,
    password: String,
    host: String,
    port: u16,
    database: String,
) -> Result<PgPool, sqlx::Error> {
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
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
) -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::get_game,
            routes::get_game_dlcs,
            routes::get_game_finishes,
            routes::get_game_logs,
            routes::get_game_tags,
            routes::get_games,
            routes::post_game,
            routes::post_game_finish,
            routes::post_game_log,
            routes::put_game,
            routes::put_game_dlc,
            routes::delete_game,
            routes::remove_game_dlc,
            routes::delete_game_finish,
            routes::delete_game_log,
            routes::get_dlc,
            routes::get_dlc_base_game,
            routes::get_dlc_finishes,
            routes::get_dlcs,
            routes::post_dlc,
            routes::post_dlc_finish,
            routes::put_dlc,
            routes::delete_dlc,
            routes::delete_dlc_finish,
            routes::get_platform,
            routes::get_platforms,
            routes::post_platform,
            routes::put_platform,
            routes::delete_platform,
            routes::get_tag,
            routes::get_tag_games,
            routes::get_tags,
            routes::post_tag,
            routes::post_tag_game,
            routes::put_tag,
            routes::delete_tag,
            routes::delete_tag_game,
            routes::get_current_user,
            routes::post_user,
            routes::change_password,
            routes::token,
            routes::is_alive,
        ),
        components(schemas(
            models::GameDTO,
            models::NewGameDTO,
            models::GameStatus,
            models::GameLogDTO,
            models::DLCDTO,
            models::NewDLCDTO,
            models::PlatformDTO,
            models::NewPlatformDTO,
            models::TagDTO,
            models::NewTagDTO,
            models::UserDTO,
            models::NewUserDTO,
            models::PasswordChangeDTO,
            models::TokenRequest,
            models::TokenResponse,
            models::ErrorMessage,
        )),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

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

    // Make instance variable of ApiDoc so all worker threads get the same instance.
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(game_collection_server::auth::token_validator);

        App::new()
            .app_data(web::Data::new(db_pool.clone())) // TODO Wrap dbpool
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
                        .service(routes::get_game_dlcs)
                        .service(routes::get_game_finishes)
                        .service(routes::get_game_logs)
                        .service(routes::get_game_tags)
                        .service(routes::get_games)
                        .service(routes::post_game)
                        .service(routes::post_game_finish)
                        .service(routes::post_game_log)
                        .service(routes::put_game)
                        .service(routes::put_game_dlc)
                        .service(routes::delete_game)
                        .service(routes::remove_game_dlc)
                        .service(routes::delete_game_finish)
                        .service(routes::delete_game_log)
                        // DLCs
                        .service(routes::get_dlc)
                        .service(routes::get_dlc_base_game)
                        .service(routes::get_dlc_finishes)
                        .service(routes::get_dlcs)
                        .service(routes::post_dlc)
                        .service(routes::post_dlc_finish)
                        .service(routes::put_dlc)
                        .service(routes::delete_dlc)
                        .service(routes::delete_dlc_finish)
                        // Platforms
                        .service(routes::get_platform)
                        .service(routes::get_platforms)
                        .service(routes::post_platform)
                        .service(routes::put_platform)
                        .service(routes::delete_platform)
                        // Tags
                        .service(routes::get_tag)
                        .service(routes::get_tag_games)
                        .service(routes::get_tags)
                        .service(routes::post_tag)
                        .service(routes::post_tag_game)
                        .service(routes::put_tag)
                        .service(routes::delete_tag)
                        .service(routes::delete_tag_game)
                        // Users
                        .service(routes::get_current_user)
                        .service(routes::post_user)
                        .service(routes::change_password),
                ),
            )
            // Authentication
            .service(web::scope("/auth").service(routes::token))
            // Health check
            .service(routes::is_alive)
            // OpenAPI
            .service(
                SwaggerUi::new("/api-docs/{_:.*}")
                    .url("/api-docs/public-api.json", openapi.clone()),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
