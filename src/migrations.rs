use std::path::Path;

use sqlx::PgPool;

use crate::models::NewUserDTO;
use crate::services::UserService;

pub async fn apply_migrations(pool: &PgPool) {
    let migrator = sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .expect("Could not load database migrations.");
    migrator
        .run(pool)
        .await
        .expect("Could not apply database migrations.");

    log::info!("Database migrations applied.");
}

pub async fn check_admin_user(user_service: &UserService) {
    let exists_admin = user_service
        .exists_admin_user()
        .await
        .expect("Could not check if admin user exists");
    match exists_admin {
        true => log::info!("Database admin present."),
        false => {
            let admin_user = user_service
                .create_user(
                    NewUserDTO {
                        username: Some(String::from("admin")),
                    },
                    "admin",
                )
                .await
                .expect("Could not create admin user");
            user_service
                .promote_user(&admin_user.id)
                .await
                .expect("Could not promote admin user");

            log::info!(
                "Database admin not present, created 'admin' user with default 'admin' password. PLEASE CHANGE PASSWORD."
            );
        }
    }
}
