use sqlx::PgPool;

use crate::entities::{User, UserSearch};
use crate::errors::ApiErrors;
use crate::models::{NewUserDTO, PasswordChangeDTO, SearchDTO, UserDTO, UserPageResult};
use crate::repository::user_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_get_result_raw,
    handle_not_found_result, handle_query_mapping, handle_result, handle_update_result,
    update_merged,
};

pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<UserDTO, ApiErrors> {
    let repository_result = user_repository::find_by_id(pool, user_id).await;
    handle_get_result(repository_result)
}

pub async fn search_users(
    pool: &PgPool,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<UserPageResult, ApiErrors> {
    let search = handle_query_mapping::<UserDTO, UserSearch>(search, quicksearch)?;
    let find_result = user_repository::search_all(pool, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_user(
    pool: &PgPool,
    user: NewUserDTO,
    password: &str,
) -> Result<UserDTO, ApiErrors> {
    create_merged(
        user,
        async move |created_user_id| get_user(pool, created_user_id).await,
        async move |user_to_create| {
            let exists_result = user_repository::exists_with_unique(pool, &user_to_create).await;
            handle_already_exists_result::<UserDTO>(exists_result)?;

            let password_hash = crate::auth::hash_password(password)
                .map_err(|_| ApiErrors::UnknownError(String::from("Password hashing error.")))?;
            let create_result =
                user_repository::create(pool, &user_to_create, &password_hash).await;
            handle_create_result::<i32, UserDTO>(create_result)
        },
    )
    .await
}

pub async fn update_user(pool: &PgPool, user_id: i32, user: NewUserDTO) -> Result<(), ApiErrors> {
    update_merged(
        user,
        async move || get_user(pool, user_id).await,
        async move |user_to_update| {
            let exists_result =
                user_repository::exists_with_unique_except_id(pool, &user_to_update, user_id).await;
            handle_already_exists_result::<UserDTO>(exists_result)?;

            let update_result = user_repository::update_by_id(pool, user_id, &user_to_update).await;
            handle_update_result::<i32, UserDTO>(update_result)
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

pub async fn promote_user(pool: &PgPool, user_id: i32) -> Result<(), ApiErrors> {
    change_user_admin(pool, user_id, true).await
}

pub async fn demote_user(pool: &PgPool, user_id: i32) -> Result<(), ApiErrors> {
    // First check if there would be admins left
    let exists_more_admins_result =
        user_repository::exists_with_admin_except_id(pool, user_id).await;
    let exists_more_admins = handle_result::<bool, UserDTO>(exists_more_admins_result)?;
    if !exists_more_admins {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Cannot demote only admin left",
        )));
    }

    change_user_admin(pool, user_id, false).await
}

async fn change_user_admin(pool: &PgPool, user_id: i32, admin: bool) -> Result<(), ApiErrors> {
    exists_user(pool, user_id).await?;

    let update_result = user_repository::update_admin(pool, user_id, admin).await;
    handle_update_result::<i32, UserDTO>(update_result)
}

pub async fn delete_user(pool: &PgPool, user_id: i32) -> Result<(), ApiErrors> {
    exists_user(pool, user_id).await?;

    let delete_result = user_repository::delete_by_id(pool, user_id).await;
    handle_action_result::<UserDTO>(delete_result)
}

pub async fn exists_user(pool: &PgPool, user_id: i32) -> Result<(), ApiErrors> {
    let exists_result = user_repository::exists_by_id(pool, user_id).await;
    handle_not_found_result::<UserDTO>(exists_result)
}
