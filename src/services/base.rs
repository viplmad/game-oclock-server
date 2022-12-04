use std::future::Future;

use crate::entities::SearchResult;
use crate::errors::{error_message_builder, ApiErrors, FieldMappingError, RepositoryError};
use crate::models::{FilterDTO, Merge, ModelInfo, SearchDTO, SearchResultDTO};

fn handle_result<E, T>(repository_result: Result<E, RepositoryError>) -> Result<E, ApiErrors>
where
    T: ModelInfo,
{
    repository_result.map_err(|err| {
        ApiErrors::UnknownError(error_message_builder::inner_error(
            &error_message_builder::database_error(T::MODEL_NAME),
            &err.0,
        ))
    })
}

pub(super) fn handle_get_result_raw<E, T>(
    repository_result: Result<Option<E>, RepositoryError>,
) -> Result<E, ApiErrors>
where
    T: ModelInfo,
{
    handle_result::<Option<E>, T>(repository_result)?.ok_or_else(|| {
        ApiErrors::NotFound(error_message_builder::not_found(
            T::MODEL_NAME,
            T::ID_FIELDS,
        ))
    })
}

pub(super) fn handle_get_result<E, T>(
    repository_result: Result<Option<E>, RepositoryError>,
) -> Result<T, ApiErrors>
where
    T: From<E> + ModelInfo,
{
    let entity: E = handle_get_result_raw::<E, T>(repository_result)?;
    Ok(T::from(entity))
}

pub(super) fn handle_get_list_result_raw<E, T>(
    repository_result: Result<Vec<E>, RepositoryError>,
) -> Result<Vec<E>, ApiErrors>
where
    T: ModelInfo,
{
    handle_result::<Vec<E>, T>(repository_result)
}

pub(super) fn handle_get_list_result<E, T>(
    repository_result: Result<Vec<E>, RepositoryError>,
) -> Result<Vec<T>, ApiErrors>
where
    T: From<E> + ModelInfo,
{
    let entity_list: Vec<E> = handle_get_list_result_raw::<E, T>(repository_result)?;
    Ok(entity_list.into_iter().map(T::from).collect())
}

pub(super) fn handle_get_list_paged_result<E, T>(
    repository_result: Result<SearchResult<E>, RepositoryError>,
) -> Result<SearchResultDTO<T>, ApiErrors>
where
    T: From<E> + ModelInfo,
{
    let entity_search = handle_result::<SearchResult<E>, T>(repository_result)?;
    Ok(SearchResultDTO {
        data: entity_search.data.into_iter().map(T::from).collect(),
        page: entity_search.page,
        size: entity_search.size,
    })
}

pub(super) fn handle_create_result<I, T>(
    repository_result: Result<I, RepositoryError>,
) -> Result<I, ApiErrors>
where
    T: ModelInfo,
{
    handle_result::<I, T>(repository_result)
}

pub(super) fn handle_update_result<I, T>(
    repository_result: Result<I, RepositoryError>,
) -> Result<(), ApiErrors>
where
    T: ModelInfo,
{
    handle_result::<I, T>(repository_result).map(|_| ())
}

pub(super) fn handle_action_result<T>(
    repository_result: Result<(), RepositoryError>,
) -> Result<(), ApiErrors>
where
    T: ModelInfo,
{
    handle_result::<(), T>(repository_result)
}

pub(super) fn handle_already_exists_result<T>(
    repository_result: Result<bool, RepositoryError>,
) -> Result<(), ApiErrors>
where
    T: ModelInfo,
{
    let exists = handle_result::<bool, T>(repository_result)?;
    match exists {
        true => Err(ApiErrors::AlreadyExists(
            error_message_builder::already_exists(T::MODEL_NAME, T::UNIQUE_FIELDS),
        )),
        false => Ok(()),
    }
}

pub(super) fn handle_not_found_result<T>(
    repository_result: Result<bool, RepositoryError>,
) -> Result<(), ApiErrors>
where
    T: ModelInfo,
{
    let exists = handle_result::<bool, T>(repository_result)?;
    match exists {
        true => Ok(()),
        false => Err(ApiErrors::NotFound(error_message_builder::not_found(
            T::MODEL_NAME,
            T::ID_FIELDS,
        ))),
    }
}

pub(super) async fn create_merged<E, T, N, I, GF, CF>(
    new: N,
    get_function: impl FnOnce(I) -> GF,
    create_function: impl FnOnce(E) -> CF,
) -> Result<T, ApiErrors>
where
    T: From<E> + Merge<N> + Default + ModelInfo,
    E: From<T>,
    GF: Future<Output = Result<T, ApiErrors>>,
    CF: Future<Output = Result<I, ApiErrors>>,
{
    let merged_new: T = T::merge_with_default(new);
    let entity_to_create: E = E::from(merged_new);

    let created_id: I = create_function(entity_to_create).await?;

    get_function(created_id).await.map_err(|err| match err {
        ApiErrors::NotFound(_) => {
            ApiErrors::NotFound(error_message_builder::created_but_error_get(T::MODEL_NAME))
        }
        other => other,
    })
}

pub(super) async fn update_merged<E, T, N, GF, UF>(
    update: N,
    get_function: impl Fn() -> GF,
    update_function: impl FnOnce(E) -> UF,
) -> Result<T, ApiErrors>
where
    T: From<E> + Merge<N> + ModelInfo,
    E: From<T>,
    GF: Future<Output = Result<T, ApiErrors>>,
    UF: Future<Output = Result<(), ApiErrors>>,
{
    let current: T = get_function().await?;

    let merged_update: T = T::merge(current, update);
    let entity_to_update: E = E::from(merged_update);

    update_function(entity_to_update).await?;

    get_function().await.map_err(|err| match err {
        ApiErrors::NotFound(_) => {
            ApiErrors::NotFound(error_message_builder::updated_but_error_get(T::MODEL_NAME))
        }
        other => other,
    })
}

pub(super) fn handle_query_mapping<T, S>(
    mut search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<S, ApiErrors>
where
    T: ModelInfo,
    S: TryFrom<SearchDTO, Error = FieldMappingError>,
{
    add_quicksearch::<T>(&mut search, quicksearch);

    S::try_from(search).map_err(|err| {
        ApiErrors::InvalidParameter(error_message_builder::field_not_found(
            T::MODEL_NAME,
            &err.0,
        ))
    })
}

fn add_quicksearch<T>(search: &mut SearchDTO, quicksearch: Option<String>)
where
    T: ModelInfo,
{
    if let Some(quicksearch_value) = quicksearch {
        let mut quicksearch_filters: Vec<FilterDTO> = T::UNIQUE_FIELDS
            .iter()
            .map(move |field| crate::models::FilterDTO {
                field: field.to_string(),
                operator: crate::models::OperatorType::Contains,
                value: crate::models::SearchValue::Value(quicksearch_value.clone()),
                chain_operator: Some(crate::models::ChainOperatorType::Or),
            })
            .collect();

        if let Some(filters) = &mut search.filter {
            quicksearch_filters.append(filters)
        }
        search.filter = Some(quicksearch_filters);
    }
}
