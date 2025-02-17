use uuid::Uuid;

use crate::entities::{Genre, GenreSearch};
use crate::errors::ApiErrors;
use crate::models::{GenreDTO, GenrePageResult, NewGenreDTO, SearchDTO};
use crate::repository::GenreRepository;

use super::helpers::{
    create_merged, handle_action_result, handle_already_exists_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

#[derive(Clone)]
pub struct GenreService {
    repository: GenreRepository,
}

impl GenreService {
    pub fn with(repository: GenreRepository) -> Self {
        Self { repository }
    }
}

impl GenreService {
    pub async fn get_genre(&self, user_id: &Uuid, id: &Uuid) -> Result<GenreDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, id).await;
        handle_get_result(find_result)
    }

    pub async fn search_genres(
        &self,
        user_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<GenrePageResult, ApiErrors> {
        let search = handle_query_mapping::<GenreDTO, GenreSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn create_genre(
        &self,
        user_id: &Uuid,
        genre: NewGenreDTO,
    ) -> Result<GenreDTO, ApiErrors> {
        let new_id = crate::uuid_utils::new_model_uuid();
        create_merged(
            genre,
            async move || self.get_genre(user_id, &new_id).await,
            async move |mut genre_to_create: Genre| {
                let exists_result = self
                    .repository
                    .exists_by_name(user_id, &genre_to_create.name)
                    .await;
                handle_already_exists_result::<GenreDTO>(exists_result)?;

                genre_to_create.user_id = user_id.clone();
                genre_to_create.id = new_id.clone();
                genre_to_create.added_datetime = crate::date_utils::now();
                genre_to_create.updated_datetime = crate::date_utils::now();
                let create_result = self.repository.create(&genre_to_create).await;
                handle_action_result::<GenreDTO>(create_result)
            },
        )
        .await
    }

    pub async fn update_genre(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        genre: NewGenreDTO,
    ) -> Result<(), ApiErrors> {
        update_merged(
            genre,
            async move || self.get_genre(user_id, id).await,
            async move |mut genre_to_update: Genre| {
                let exists_result = self
                    .repository
                    .exists_by_name_except_id(user_id, &genre_to_update.name, id)
                    .await;
                handle_already_exists_result::<GenreDTO>(exists_result)?;

                genre_to_update.user_id = user_id.clone();
                genre_to_update.id = id.clone();
                genre_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update(&genre_to_update).await;
                handle_update_result::<GenreDTO>(update_result)
            },
        )
        .await
    }

    pub async fn delete_genre(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let delete_result = self.repository.delete_by_id(user_id, id).await;
        handle_action_result::<GenreDTO>(delete_result)
    }

    pub async fn exists_genre(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, id).await;
        handle_not_found_result::<GenreDTO>(exists_result)
    }
}
