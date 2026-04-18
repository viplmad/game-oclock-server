use std::future::Future;

use chrono::{DateTime, NaiveDate, Utc};

use crate::entities::{AggregateResult, PageResult};
use crate::errors::{
    ApiErrors, MappingError, RepositoryError, SearchErrors, error_message_builder,
};
use crate::models::{
    AggregateResultDTO, AggregateSearchDTO, DurationDef, FilterDTO, ListSearchDTO, Merge,
    ModelInfo, PageResultDTO,
};

pub fn handle_result<E, T>(repository_result: Result<E, RepositoryError>) -> Result<E, ApiErrors>
where
    T: ModelInfo,
{
    repository_result
        .map_err(|_| ApiErrors::UnknownError(error_message_builder::database_error(T::MODEL_NAME)))
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
    let entity = handle_get_result_raw::<E, T>(repository_result)?;
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

pub(super) fn handle_get_list_paged_result<E, T>(
    repository_result: Result<PageResult<E>, SearchErrors>,
) -> Result<PageResultDTO<T>, ApiErrors>
where
    T: From<E> + ModelInfo,
{
    let entity_search = repository_result.map_err(|err| match err {
        SearchErrors::Mapping(map_err) => {
            ApiErrors::InvalidParameter(error_message_builder::inner_error(
                &error_message_builder::database_error(T::MODEL_NAME),
                &map_err.0,
            ))
        }
        SearchErrors::Repository(_) => {
            ApiErrors::UnknownError(error_message_builder::database_error(T::MODEL_NAME))
        }
    })?;
    Ok(PageResultDTO {
        data: entity_search.data.into_iter().map(T::from).collect(),
        page: entity_search.page,
        size: entity_search.size,
    })
}

pub(super) fn handle_get_count_result<T>(
    repository_result: Result<u64, SearchErrors>,
) -> Result<u64, ApiErrors>
where
    T: ModelInfo,
{
    repository_result.map_err(|err| match err {
        SearchErrors::Mapping(map_err) => {
            ApiErrors::InvalidParameter(error_message_builder::inner_error(
                &error_message_builder::database_error(T::MODEL_NAME),
                &map_err.0,
            ))
        }
        SearchErrors::Repository(_) => {
            ApiErrors::UnknownError(error_message_builder::database_error(T::MODEL_NAME))
        }
    })
}

pub(super) fn handle_get_aggregate_result<T>(
    repository_result: Result<AggregateResult, SearchErrors>,
) -> Result<AggregateResultDTO, ApiErrors>
where
    T: ModelInfo,
{
    let entity_search = repository_result.map_err(|err| match err {
        SearchErrors::Mapping(map_err) => {
            ApiErrors::InvalidParameter(error_message_builder::inner_error(
                &error_message_builder::database_error(T::MODEL_NAME),
                &map_err.0,
            ))
        }
        SearchErrors::Repository(_) => {
            ApiErrors::UnknownError(error_message_builder::database_error(T::MODEL_NAME))
        }
    })?;
    Ok(match entity_search {
        AggregateResult::Integer(i) => AggregateResultDTO::Integer(i),
        AggregateResult::Duration(d) => AggregateResultDTO::Duration(DurationDef::from(d)),
    })
}

pub(super) fn handle_update_result<T>(
    repository_result: Result<(), RepositoryError>,
) -> Result<(), ApiErrors>
where
    T: ModelInfo,
{
    handle_result::<(), T>(repository_result)
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

// TODO
pub(super) async fn create_merged2<E, T, N, CF>(
    new: N,
    create_function: impl FnOnce(E) -> CF,
) -> Result<(), ApiErrors>
where
    T: Merge<N> + Default,
    E: From<T>,
    CF: Future<Output = Result<(), ApiErrors>>,
{
    let merged_new = T::merge_with_default(new);
    let entity_to_create = E::from(merged_new);

    create_function(entity_to_create).await
}

pub(super) async fn create_merged<E, T, N, GF, CF>(
    new: N,
    get_function: impl FnOnce() -> GF,
    create_function: impl FnOnce(E) -> CF,
) -> Result<T, ApiErrors>
where
    T: From<E> + Merge<N> + Default + ModelInfo,
    E: From<T>,
    GF: Future<Output = Result<T, ApiErrors>>,
    CF: Future<Output = Result<(), ApiErrors>>,
{
    let merged_new = T::merge_with_default(new);
    let entity_to_create = E::from(merged_new);

    create_function(entity_to_create).await?;

    get_function().await.map_err(|err| match err {
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
) -> Result<(), ApiErrors>
where
    T: Merge<N>,
    E: From<T>,
    GF: Future<Output = Result<T, ApiErrors>>,
    UF: Future<Output = Result<(), ApiErrors>>,
{
    let current = get_function().await?;

    let merged_update = T::merge(current, update);
    let entity_to_update = E::from(merged_update);

    update_function(entity_to_update).await
}

pub(super) fn check_optional_start_end(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<(), ApiErrors> {
    if start_date.is_none() && end_date.is_none() {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Start date and end date cannot be empty",
        )));
    }
    if let Some(start) = start_date {
        if let Some(end) = end_date {
            check_start_end(start, end)?;
        }
    }
    Ok(())
}

pub(super) fn check_start_end(start_date: NaiveDate, end_date: NaiveDate) -> Result<(), ApiErrors> {
    if start_date > end_date {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Start date must be previous than end date",
        )));
    }
    Ok(())
}

pub(super) fn optional_start_end_to_datetime(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
    let start_datetime = start_date.map(crate::date_utils::date_at_start_of_day);
    let end_datetime = end_date.map(crate::date_utils::date_at_midnight);
    (start_datetime, end_datetime)
}

pub(super) fn start_end_to_datetime(
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let start_datetime = crate::date_utils::date_at_start_of_day(start_date);
    let end_datetime = crate::date_utils::date_at_midnight(end_date);
    (start_datetime, end_datetime)
}

pub(super) fn handle_list_search_mapping<T, S>(
    mut search: ListSearchDTO,
    quicksearch: Option<String>,
) -> Result<S, ApiErrors>
where
    T: ModelInfo,
    S: TryFrom<ListSearchDTO, Error = MappingError>,
{
    add_quicksearch::<T>(&mut search.filter, quicksearch);
    handle_search_mapping::<T, _, _>(search)
}

pub(super) fn handle_aggregate_search_mapping<T, S>(
    mut search: AggregateSearchDTO,
    quicksearch: Option<String>,
) -> Result<S, ApiErrors>
where
    T: ModelInfo,
    S: TryFrom<AggregateSearchDTO, Error = MappingError>,
{
    add_quicksearch::<T>(&mut search.filter, quicksearch);
    handle_search_mapping::<T, _, _>(search)
}

fn handle_search_mapping<T, SS, S>(search: SS) -> Result<S, ApiErrors>
where
    T: ModelInfo,
    S: TryFrom<SS, Error = MappingError>,
{
    S::try_from(search).map_err(|err| {
        ApiErrors::InvalidParameter(error_message_builder::field_not_found(
            T::MODEL_NAME,
            &err.0,
        ))
    })
}

fn add_quicksearch<T>(filter: &mut Option<Vec<FilterDTO>>, quicksearch: Option<String>)
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

        if let Some(filters) = filter {
            quicksearch_filters.append(filters)
        }
        *filter = Some(quicksearch_filters);
    }
}
