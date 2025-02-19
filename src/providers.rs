use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};

/// Connection options to Sqlx Postgres.
#[derive(Debug)]
pub struct SqlxPostgresPoolBuilder;

impl SqlxPostgresPoolBuilder {
    pub async fn from_env() -> Result<PgPool, sqlx::Error> {
        let host = std::env::var("DB_HOST")
            .expect("Database host not set. Set through 'DB_HOST' environemnt variable.");
        let port = std::env::var("DB_PORT")
            .expect("Database port not set. Set through 'DB_PORT' environemnt variable.")
            .parse()
            .expect("Database port is not a number.");
        let database = std::env::var("DB_DATABASE")
            .expect("Database not set. Set through 'DB_DATABASE' environemnt variable.");
        let user = std::env::var("DB_USER")
            .expect("Database user not set. Set through 'DB_USER' environemnt variable.");
        let password = std::env::var("DB_PASSWORD")
            .expect("Database password not set. Set through 'DB_PASSWORD' environemnt variable.");

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
}
