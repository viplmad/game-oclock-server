use uuid::Uuid;

use crate::entities::{ExternalMedia, Media, MediaAggregateSearch, MediaListSearch, MediaState};
use crate::errors::ApiErrors;
use crate::models::{
    AggregateResultDTO, AggregateSearchDTO, ExternalMediaIdDTO, ListSearchDTO, MediaDTO,
    MediaDataDTO, MediaPageResult, MediaStateDTO, MediaStatus, NewManualMediaDTO, NewMediaDTO,
    NewMediaStateDTO, PotentialMediaDTO,
};
use crate::repository::MediaRepository;

use super::MediaExternalService;
use super::helpers::{
    create_merged, handle_action_result, handle_aggregate_search_mapping,
    handle_already_exists_result, handle_get_aggregate_result, handle_get_list_paged_result,
    handle_get_list_result_raw, handle_get_result, handle_list_search_mapping,
    handle_not_found_result, handle_result, handle_update_result, update_merged,
};

#[derive(Clone)]
pub struct MediaService {
    external_service: MediaExternalService,
    repository: MediaRepository,
}

impl MediaService {
    pub fn with(external_service: MediaExternalService, repository: MediaRepository) -> Self {
        Self {
            external_service,
            repository,
        }
    }
}

impl MediaService {
    pub async fn get_media(&self, user_id: &Uuid, id: &Uuid) -> Result<MediaDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, id).await;
        handle_get_result(find_result)
    }

    async fn get_media_data(&self, id: &Uuid) -> Result<MediaDataDTO, ApiErrors> {
        let find_result = self.repository.find_data_by_id(id).await;
        handle_get_result(find_result)
    }

    async fn get_media_state(&self, user_id: &Uuid, id: &Uuid) -> Result<MediaStateDTO, ApiErrors> {
        let find_result = self.repository.find_state_by_id(user_id, id).await;
        handle_get_result(find_result)
    }

    async fn get_media_external_primary(&self, id: &Uuid) -> Result<ExternalMediaIdDTO, ApiErrors> {
        let find_result = self.repository.find_external_by_id(id).await;
        handle_get_result(find_result)
    }

    pub async fn get_media_by_external(
        &self,
        user_id: &Uuid,
        external: &ExternalMediaIdDTO,
    ) -> Result<MediaDTO, ApiErrors> {
        let find_result = self
            .repository
            .find_by_external(user_id, &external.source, &external.id)
            .await;
        handle_get_result(find_result)
    }

    async fn get_media_data_by_external(
        &self,
        external: &ExternalMediaIdDTO,
    ) -> Result<MediaDataDTO, ApiErrors> {
        let find_result = self
            .repository
            .find_data_by_external(&external.source, &external.id)
            .await;
        handle_get_result(find_result)
    }

    pub async fn search_medias(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<MediaPageResult, ApiErrors> {
        let search = handle_list_search_mapping::<MediaDTO, MediaListSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn aggregate_medias(
        &self,
        user_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        let search =
            handle_aggregate_search_mapping::<MediaDTO, MediaAggregateSearch>(search, quicksearch)?;
        let aggregate_result = self.repository.aggregate_all(user_id, search).await;
        handle_get_aggregate_result::<MediaDTO>(aggregate_result)
    }

    pub async fn search_external_medias(
        &self,
        user_id: &Uuid,
        source: &str,
        quicksearch: &str,
    ) -> Result<Vec<PotentialMediaDTO>, ApiErrors> {
        let externals = self
            .external_service
            .search(source, quicksearch, 20)
            .await?;

        let external_ids = externals
            .iter()
            .map(|(external_id, _)| (external_id.source.clone(), external_id.id.clone()))
            .collect();
        let find_states_result = self
            .repository
            .find_all_states_by_external_ids(user_id, &external_ids)
            .await;
        let states = handle_get_list_result_raw::<_, MediaDTO>(find_states_result)?;

        Ok(externals
            .into_iter()
            .map(|(external, mut media)| {
                let state = states.iter().find(|state| {
                    state.external_source == external.source && state.external_id == external.id
                });

                // Update with existing id
                media.id = state.map(|s| s.media_id.clone());

                PotentialMediaDTO {
                    external,
                    media,
                    state: state.map(MediaStateDTO::from),
                }
            })
            .collect())
    }

    pub async fn create_media(
        &self,
        user_id: &Uuid,
        media: NewMediaDTO,
    ) -> Result<Uuid, ApiErrors> {
        check_media_state_rating(&media.state)?;

        match media.media {
            crate::models::NewMediaValue::External(external) => {
                self.create_media_from_external(user_id, external, media.state)
                    .await
            }
            crate::models::NewMediaValue::Manual(manual) => {
                self.create_media_from_manual(user_id, manual, media.state)
                    .await
            }
        }
    }

    async fn create_media_from_external(
        &self,
        user_id: &Uuid,
        external: ExternalMediaIdDTO,
        state: NewMediaStateDTO,
    ) -> Result<Uuid, ApiErrors> {
        let existing_media_result = self.get_media_data_by_external(&external).await;

        match existing_media_result {
            Ok(existing_media) => {
                let existing_id = existing_media.id;

                self.create_media_state(user_id, &existing_id, state)
                    .await?;

                Ok(existing_id)
            }
            Err(err) => match err {
                ApiErrors::NotFound(_) => {
                    let media = self.external_service.get(&external).await?;

                    let new_id = crate::uuid_utils::new_model_uuid();

                    self.create_media_data(&new_id, media).await?;
                    self.create_media_external(&new_id, external).await?;
                    self.create_media_state(user_id, &new_id, state).await?;

                    Ok(new_id)
                }
                _ => Err(err),
            },
        }
    }

    async fn create_media_from_manual(
        &self,
        user_id: &Uuid,
        media: NewManualMediaDTO,
        state: NewMediaStateDTO,
    ) -> Result<Uuid, ApiErrors> {
        let new_id = crate::uuid_utils::new_model_uuid();

        self.create_media_data(&new_id, media).await?;
        self.create_media_state(user_id, &new_id, state).await?;

        Ok(new_id)
    }

    async fn create_media_data(
        &self,
        id: &Uuid,
        media: NewManualMediaDTO,
    ) -> Result<(), ApiErrors> {
        create_merged::<Media, MediaDataDTO, NewManualMediaDTO, _>(
            media,
            async move |mut media_to_create: Media| {
                let exists_result = self
                    .repository
                    .exists_by_title_and_edition(&media_to_create.title, &media_to_create.edition)
                    .await;
                handle_already_exists_result::<MediaDTO>(exists_result)?;

                media_to_create.id = id.clone();
                media_to_create.added_datetime = crate::date_utils::now();
                media_to_create.updated_datetime = crate::date_utils::now();
                let create_result = self.repository.create_data(&media_to_create).await;
                handle_action_result::<MediaDTO>(create_result)
            },
        )
        .await
    }

    async fn create_media_state(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        state: NewMediaStateDTO,
    ) -> Result<(), ApiErrors> {
        create_merged::<MediaState, MediaStateDTO, NewMediaStateDTO, _>(
            state,
            async move |mut state_to_create: MediaState| {
                state_to_create.user_id = user_id.clone();
                state_to_create.media_id = id.clone();
                state_to_create.added_datetime = crate::date_utils::now();
                state_to_create.updated_datetime = crate::date_utils::now();

                let create_result = self.repository.create_state(&state_to_create).await;
                handle_action_result::<MediaDTO>(create_result)
            },
        )
        .await
    }

    async fn create_media_external(
        &self,
        id: &Uuid,
        external: ExternalMediaIdDTO,
    ) -> Result<(), ApiErrors> {
        let create_result = self
            .repository
            .create_external(&ExternalMedia {
                primary: true,
                media_id: id.clone(),
                external_source: external.source.clone(),
                external_id: external.id.clone(),
            })
            .await;
        handle_action_result::<MediaDTO>(create_result)
    }

    pub async fn update_media(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        media: NewMediaDTO,
    ) -> Result<(), ApiErrors> {
        check_media_state_rating(&media.state)?;

        match media.media {
            crate::models::NewMediaValue::External(external) => {
                self.update_media_from_external(user_id, id, external, media.state)
                    .await
            }
            crate::models::NewMediaValue::Manual(manual) => {
                self.update_media_from_manual(user_id, id, manual, media.state)
                    .await
            }
        }
    }

    async fn update_media_from_external(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        external: ExternalMediaIdDTO,
        state: NewMediaStateDTO,
    ) -> Result<(), ApiErrors> {
        let current = self.get_media_external_primary(id).await?;
        if current.source == external.source && current.id == external.id {
            return Err(ApiErrors::AlreadyExists(String::from(
                "Media is already linked to this external source, use sync to update data",
            )));
        }

        let exists_result = self
            .repository
            .exists_by_external_source_and_id_except_id(&external.source, &external.id, id)
            .await;
        handle_already_exists_result::<MediaDTO>(exists_result)?;

        let media = self.external_service.get(&external).await?;

        self.update_media_data(id, media).await?;
        self.update_media_external(id, external).await?;
        self.update_media_state(user_id, id, state).await
    }

    async fn update_media_from_manual(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        media: NewManualMediaDTO,
        state: NewMediaStateDTO,
    ) -> Result<(), ApiErrors> {
        self.update_media_data(id, media).await?;
        self.update_media_state(user_id, id, state).await
    }

    async fn update_media_data(
        &self,
        id: &Uuid,
        manual: NewManualMediaDTO,
    ) -> Result<(), ApiErrors> {
        update_merged::<Media, MediaDataDTO, NewManualMediaDTO, _, _>(
            manual,
            async move || self.get_media_data(id).await,
            async move |mut media_to_update: Media| {
                let exists_result = self
                    .repository
                    .exists_by_title_and_edition_except_id(
                        &media_to_update.title,
                        &media_to_update.edition,
                        id,
                    )
                    .await;
                handle_already_exists_result::<MediaDTO>(exists_result)?;

                media_to_update.id = id.clone();
                media_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update_data(&media_to_update).await;
                handle_update_result::<MediaDTO>(update_result)
            },
        )
        .await
    }

    async fn update_media_state(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        state: NewMediaStateDTO,
    ) -> Result<(), ApiErrors> {
        update_merged::<MediaState, MediaStateDTO, NewMediaStateDTO, _, _>(
            state,
            async move || self.get_media_state(user_id, id).await,
            async move |mut state_to_update: MediaState| {
                state_to_update.user_id = user_id.clone();
                state_to_update.media_id = id.clone();
                state_to_update.updated_datetime = crate::date_utils::now();

                let update_result = self.repository.update_state(&state_to_update).await;
                handle_update_result::<MediaDTO>(update_result)
            },
        )
        .await
    }

    async fn update_media_external(
        &self,
        id: &Uuid,
        external: ExternalMediaIdDTO,
    ) -> Result<(), ApiErrors> {
        let update_result = self
            .repository
            .update_external(&ExternalMedia {
                primary: true,
                media_id: id.clone(),
                external_source: external.source.clone(),
                external_id: external.id.clone(),
            })
            .await;
        handle_update_result::<MediaDTO>(update_result)
    }

    pub async fn update_media_status(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        status: MediaStatus,
    ) -> Result<(), ApiErrors> {
        self.exists_media_state(user_id, id).await?;

        let update_result = self
            .repository
            .update_status(user_id, id, i16::from(status))
            .await;
        handle_action_result::<MediaDTO>(update_result)
    }

    pub async fn delete_media(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        self.delete_media_state(user_id, id).await?;

        // Delete media if no other states are found
        let existing_states_result = self.repository.exists_any_state_by_id(id).await;
        let existing_states = handle_result::<bool, MediaDTO>(existing_states_result)?;
        if !existing_states {
            self.delete_media_data(id).await?;
            self.delete_media_external(id).await?;
        }

        Ok(())
    }

    async fn delete_media_data(&self, id: &Uuid) -> Result<(), ApiErrors> {
        let delete_result = self.repository.delete_data_by_id(id).await;
        handle_action_result::<MediaDTO>(delete_result)
    }

    async fn delete_media_state(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let delete_result = self.repository.delete_state_by_id(user_id, id).await;
        handle_action_result::<MediaDTO>(delete_result)
    }

    async fn delete_media_external(&self, id: &Uuid) -> Result<(), ApiErrors> {
        let delete_result = self.repository.delete_external_by_id(id).await;
        handle_action_result::<MediaDTO>(delete_result)
    }

    pub async fn exists_media(&self, id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_data_by_id(id).await;
        handle_not_found_result::<MediaDTO>(exists_result)
    }

    pub async fn exists_media_state(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_state_by_id(user_id, id).await;
        handle_not_found_result::<MediaDTO>(exists_result)
    }

    pub async fn set_media_parent(
        &self,
        id: &Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<(), ApiErrors> {
        self.exists_media(id).await?;

        if let Some(id) = &parent_id {
            // Set base media
            if id == id {
                return Err(ApiErrors::InvalidParameter(String::from(
                    "Media and base media cannot be the same",
                )));
            }

            let parent = self.get_media_data(id).await?;
            if parent.parent_id.is_some() {
                return Err(ApiErrors::InvalidParameter(String::from(
                    "Base media cannot be itself a child of another media",
                )));
            }

            let has_children = self.has_media_children(id).await?;
            if has_children {
                return Err(ApiErrors::InvalidParameter(String::from(
                    "Media cannot have children",
                )));
            }
        }
        // else unset base media

        let update_result = self.repository.update_parent_id(id, parent_id).await;
        handle_action_result::<MediaDTO>(update_result)
    }

    async fn has_media_children(&self, id: &Uuid) -> Result<bool, ApiErrors> {
        let exists_result = self.repository.exists_any_by_parent_id(id).await;
        handle_result::<_, MediaDTO>(exists_result)
    }

    pub async fn sync_media(&self, id: &Uuid) -> Result<(), ApiErrors> {
        let external = self.get_media_external_primary(id).await?;
        let new_media = self.external_service.get(&external).await?;
        self.update_media_data(id, new_media).await
    }
}

fn check_media_state_rating(state: &NewMediaStateDTO) -> Result<(), ApiErrors> {
    if state.rating.is_some_and(|rating| rating > 10) {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Rating must be between 0 and 10, both inclusive",
        )));
    }
    Ok(())
}
