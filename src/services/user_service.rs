use uuid::Uuid;

use crate::entities::{User, UserListSearch};
use crate::errors::ApiErrors;
use crate::models::{ListSearchDTO, NewUserDTO, PasswordChangeDTO, UserDTO, UserPageResult};
use crate::repository::UserRepository;

use super::helpers::{
    create_merged, handle_action_result, handle_already_exists_result, handle_get_aggregate_result,
    handle_get_list_paged_result, handle_get_result, handle_get_result_raw,
    handle_list_search_mapping, handle_not_found_result, handle_result, handle_update_result,
    update_merged,
};

const ROLE_ADMIN: &str = "ROLE_ADMIN";
const ROLE_USER: &str = "ROLE_USER";

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn with(repository: UserRepository) -> Self {
        Self { repository }
    }
}

impl UserService {
    pub async fn get_user(&self, user_id: &Uuid) -> Result<UserDTO, ApiErrors> {
        let repository_result = self.repository.find_by_id(user_id).await;
        handle_get_result(repository_result)
    }

    // For auth
    pub(super) async fn find_user_by_username(&self, username: &str) -> Result<User, ApiErrors> {
        let repository_result = self.repository.find_first_by_username(username).await;
        handle_get_result_raw::<User, UserDTO>(repository_result)
    }

    pub async fn search_users(
        &self,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<UserPageResult, ApiErrors> {
        let search = handle_list_search_mapping::<UserDTO, UserListSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn count_users(
        &self,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<u64, ApiErrors> {
        let search = handle_list_search_mapping::<UserDTO, UserListSearch>(search, quicksearch)?;
        let count_result = self.repository.count_all(search).await;
        handle_get_aggregate_result::<UserDTO>(count_result)
    }

    pub async fn create_user(
        &self,
        user: NewUserDTO,
        password: &str,
    ) -> Result<UserDTO, ApiErrors> {
        let new_id = crate::uuid_utils::new_model_uuid();
        create_merged(
            user,
            async move || self.get_user(&new_id).await,
            async move |mut user_to_create: User| {
                let exists_result = self
                    .repository
                    .exists_by_username(&user_to_create.username)
                    .await;
                handle_already_exists_result::<UserDTO>(exists_result)?;

                let password_hash = crate::auth::hash_password(password).map_err(|_| {
                    ApiErrors::UnknownError(String::from("Password hashing error."))
                })?;
                user_to_create.id = new_id.clone();
                user_to_create.password = password_hash;
                user_to_create.added_datetime = crate::date_utils::now();
                user_to_create.updated_datetime = crate::date_utils::now();
                let create_result = self.repository.create(&user_to_create).await;
                handle_action_result::<UserDTO>(create_result)
            },
        )
        .await
    }

    pub async fn update_user(&self, id: &Uuid, user: NewUserDTO) -> Result<(), ApiErrors> {
        update_merged(
            user,
            async move || self.get_user(id).await,
            async move |mut user_to_update: User| {
                let exists_result = self
                    .repository
                    .exists_by_username_except_id(&user_to_update.username, id)
                    .await;
                handle_already_exists_result::<UserDTO>(exists_result)?;

                user_to_update.id = id.clone();
                user_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update(&user_to_update).await;
                handle_update_result::<UserDTO>(update_result)
            },
        )
        .await
    }

    pub async fn change_user_password(
        &self,
        user_id: &Uuid,
        password_change: PasswordChangeDTO,
    ) -> Result<(), ApiErrors> {
        let get_result = self.repository.find_by_id(user_id).await;
        let user = handle_get_result_raw::<User, UserDTO>(get_result)?;

        let verify_pass: bool =
            crate::auth::verify_password(&password_change.current_password, &user.password)
                .map_err(|_| {
                    ApiErrors::UnknownError(String::from("Password verification failed."))
                })?;

        if verify_pass {
            let password_hash = crate::auth::hash_password(&password_change.new_password)
                .map_err(|_| ApiErrors::UnknownError(String::from("Password hashing error.")))?;

            let update_result = self
                .repository
                .update_password_by_id(user_id, &password_hash)
                .await;
            handle_update_result::<UserDTO>(update_result)
        } else {
            Err(ApiErrors::InvalidParameter(String::from("Wrong password.")))
        }
    }

    pub async fn promote_user(&self, user_id: &Uuid) -> Result<(), ApiErrors> {
        self.change_user_role(user_id, ROLE_ADMIN).await
    }

    pub async fn demote_user(&self, user_id: &Uuid) -> Result<(), ApiErrors> {
        // First check if there would be admins left
        let exists_more_admins_result = self
            .repository
            .exists_with_role_except_id(user_id, ROLE_ADMIN)
            .await;
        let exists_more_admins = handle_result::<bool, UserDTO>(exists_more_admins_result)?;
        if !exists_more_admins {
            return Err(ApiErrors::InvalidParameter(String::from(
                "Cannot demote only admin left",
            )));
        }

        self.change_user_role(user_id, ROLE_USER).await
    }

    async fn change_user_role(&self, user_id: &Uuid, role: &str) -> Result<(), ApiErrors> {
        self.exists_user(user_id).await?;

        let update_result = self.repository.update_admin_by_id(user_id, role).await;
        handle_update_result::<UserDTO>(update_result)
    }

    pub async fn delete_user(&self, user_id: &Uuid) -> Result<(), ApiErrors> {
        self.exists_user(user_id).await?;

        let delete_result = self.repository.delete_by_id(user_id).await;
        handle_action_result::<UserDTO>(delete_result)
    }

    pub async fn is_user_admin(&self, user_id: &Uuid) -> Result<bool, ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id_and_role(user_id, ROLE_ADMIN)
            .await;
        handle_result::<bool, UserDTO>(exists_result)
    }

    pub async fn exists_user(&self, user_id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id).await;
        handle_not_found_result::<UserDTO>(exists_result)
    }

    pub async fn exists_admin_user(&self) -> Result<bool, ApiErrors> {
        let exists_result = self.repository.exists_with_role(ROLE_ADMIN).await;
        handle_result::<bool, UserDTO>(exists_result)
    }
}
