use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{
    AVAILABLE_ADDED_DATETIME_ALIAS, AVAILABLE_DATE_ALIAS, AVAILABLE_UPDATED_DATETIME_ALIAS,
    LocationIden, LocationListSearch, MediaAvailable, MediaAvailableIden, MediaIden,
    MediaListSearch, SearchQuery,
};
use crate::errors::SearchErrors;

use super::search::{apply_search, apply_search_filter2};
use super::{location_query, media_query};

pub fn select_all_medias_by_location_id_order_by_date(
    user_id: &Uuid,
    location_id: &Uuid,
    search: MediaListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = media_query::select_all(user_id);

    join_media_available_by_location_id(&mut select, location_id);
    add_fields(&mut select);
    add_order_by_date(&mut select);

    apply_search(select, search)
}

pub fn count_all_medias_by_location_id_order_by_date(
    user_id: &Uuid,
    location_id: &Uuid,
    search: MediaListSearch,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = media_query::count_all(user_id);

    join_media_available_by_location_id(&mut select, location_id);

    apply_search_filter2(select, search)
}

pub fn select_all_locations_by_media_id_order_by_date(
    user_id: &Uuid,
    media_id: &Uuid,
    search: LocationListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = location_query::select_all(user_id);

    join_media_available_by_media_id(&mut select, media_id);
    add_fields(&mut select);
    add_order_by_date(&mut select);

    apply_search(select, search)
}

pub fn count_all_locations_by_media_id_order_by_date(
    user_id: &Uuid,
    media_id: &Uuid,
    search: LocationListSearch,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = location_query::count_all(user_id);

    join_media_available_by_media_id(&mut select, media_id);

    apply_search_filter2(select, search)
}

pub fn insert(media_available: &MediaAvailable) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(MediaAvailableIden::Table)
        .columns([
            MediaAvailableIden::UserId,
            MediaAvailableIden::MediaId,
            MediaAvailableIden::LocationId,
            MediaAvailableIden::Date,
            MediaAvailableIden::AddedDatetime,
            MediaAvailableIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&media_available.user_id).into(),
            crate::uuid_utils::to_string(&media_available.media_id).into(),
            crate::uuid_utils::to_string(&media_available.location_id).into(),
            media_available.date.into(),
            media_available.added_datetime.into(),
            media_available.updated_datetime.into(),
        ]);

    insert
}

pub fn delete_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    location_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(MediaAvailableIden::Table)
        .and_where(Expr::col(MediaAvailableIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(
            Expr::col(MediaAvailableIden::MediaId).eq(crate::uuid_utils::to_string(media_id)),
        )
        .and_where(
            Expr::col(MediaAvailableIden::LocationId).eq(crate::uuid_utils::to_string(location_id)),
        );

    delete
}

pub fn exists_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    location_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((MediaAvailableIden::Table, MediaAvailableIden::UserId))
        .and_where(
            Expr::col(MediaAvailableIden::MediaId).eq(crate::uuid_utils::to_string(media_id)),
        )
        .and_where(
            Expr::col(MediaAvailableIden::LocationId).eq(crate::uuid_utils::to_string(location_id)),
        );

    select
}

pub fn exists_locations_by_media_id(user_id: &Uuid, media_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((MediaAvailableIden::Table, MediaAvailableIden::UserId))
        .and_where(
            Expr::col(MediaAvailableIden::MediaId).eq(crate::uuid_utils::to_string(media_id)),
        );

    select
}

fn join_media_available_by_location_id(select: &mut SelectStatement, location_id: &Uuid) {
    select
        .left_join(
            MediaAvailableIden::Table,
            Expr::col((MediaIden::Table, MediaIden::Id))
                .equals((MediaAvailableIden::Table, MediaAvailableIden::MediaId)),
        )
        .and_where(
            Expr::col((MediaAvailableIden::Table, MediaAvailableIden::LocationId))
                .eq(crate::uuid_utils::to_string(location_id)),
        );
}

fn join_media_available_by_media_id(select: &mut SelectStatement, media_id: &Uuid) {
    select
        .left_join(
            MediaAvailableIden::Table,
            Expr::col((LocationIden::Table, LocationIden::UserId))
                .equals((MediaAvailableIden::Table, MediaAvailableIden::UserId))
                .and(
                    Expr::col((LocationIden::Table, LocationIden::Id))
                        .equals((MediaAvailableIden::Table, MediaAvailableIden::LocationId)),
                ),
        )
        .and_where(
            Expr::col((MediaAvailableIden::Table, MediaAvailableIden::MediaId))
                .eq(crate::uuid_utils::to_string(media_id)),
        );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(MediaAvailableIden::Table).and_where(
        Expr::col((MediaAvailableIden::Table, MediaAvailableIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn add_fields(select: &mut SelectStatement) {
    select
        .expr_as(
            Expr::col((MediaAvailableIden::Table, MediaAvailableIden::Date)),
            Alias::new(AVAILABLE_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((MediaAvailableIden::Table, MediaAvailableIden::AddedDatetime)),
            Alias::new(AVAILABLE_ADDED_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((
                MediaAvailableIden::Table,
                MediaAvailableIden::UpdatedDatetime,
            )),
            Alias::new(AVAILABLE_UPDATED_DATETIME_ALIAS),
        );
}

fn add_order_by_date(select: &mut SelectStatement) {
    select.order_by(
        (MediaAvailableIden::Table, MediaAvailableIden::Date),
        Order::Asc,
    );
}
