use std::path::Path;

use sqlx::PgPool;

use crate::temp_file_utils;

use crate::models::NewUserDTO;
use crate::services::users_service;

pub async fn apply_migrations(pool: &PgPool) {
    let migrator = sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .expect("Could not load database migrations.");
    migrator
        .run(pool)
        .await
        .expect("Could not apply database migrations.");

    let exists_admin = users_service::exists_admin_user(pool)
        .await
        .expect("Could not check if admin user exists");
    match exists_admin {
        true => log::info!("Database admin present."),
        false => {
            users_service::create_user(
                pool,
                NewUserDTO {
                    username: String::from("admin"),
                },
                "admin",
            )
            .await
            .expect("Could not create admin user");

            log::info!("Database admin not present, created 'admin' user with default 'admin' password. PLEASE CHANGE PASSWORD.");
        }
    }

    log::info!("Database migrations applied.");
}

pub async fn delete_old_temp_files() {
    temp_file_utils::delete_all_temp_dirs().await;

    log::info!("Old temp images deleted.");
}
