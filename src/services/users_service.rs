use sqlx::PgPool;

use crate::entities::User;
use crate::errors::ApiErrors;
use crate::models::{NewUserDTO, PasswordChangeDTO, UserDTO};
use crate::repository::user_repository;

use super::base::{
    create_merged, handle_already_exists_result, handle_create_result, handle_get_result,
    handle_get_result_raw, handle_update_result,
};

pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<UserDTO, ApiErrors> {
    let repository_result = user_repository::find_by_id(pool, user_id).await;
    handle_get_result(repository_result)
}

pub async fn create_user(pool: &PgPool, user: NewUserDTO) -> Result<UserDTO, ApiErrors> {
    let password = user.password.clone();

    create_merged(
        user,
        async move |created_user_id| get_user(pool, created_user_id).await,
        async move |user_to_create| {
            let exists_result = user_repository::exists_with_unique(pool, &user_to_create).await;
            handle_already_exists_result::<UserDTO>(exists_result)?;

            let password_hash = crate::auth::hash_password(&password)
                .map_err(|_| ApiErrors::UnknownError(String::from("Password hashing error.")))?;
            let create_result =
                user_repository::create(pool, &user_to_create, &password_hash).await;
            handle_create_result::<i32, UserDTO>(create_result)
        },
    )
    .await
}

pub async fn change_user_password(
    pool: &PgPool,
    user_id: i32,
    password_change: PasswordChangeDTO,
) -> Result<(), ApiErrors> {
    let get_result = user_repository::find_by_id(pool, user_id).await;
    let user = handle_get_result_raw::<User, UserDTO>(get_result)?;

    let verify_pass: bool =
        crate::auth::verify_password(&password_change.current_password, &user.password)
            .map_err(|_| ApiErrors::UnknownError(String::from("Password verification failed.")))?;

    if verify_pass {
        let password_hash = crate::auth::hash_password(&password_change.new_password)
            .map_err(|_| ApiErrors::UnknownError(String::from("Password hashing error.")))?;

        let update_result = user_repository::update_password(pool, user_id, &password_hash).await;
        handle_update_result::<i32, UserDTO>(update_result)
    } else {
        Err(ApiErrors::InvalidParameter(String::from("Wrong password.")))
    }
}
