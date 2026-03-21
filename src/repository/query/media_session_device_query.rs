use sea_query::{Expr, Order, SelectStatement};
use uuid::Uuid;

use crate::entities::{
    DeviceIden, DeviceSearch, MediaIden, MediaSearch, MediaSessionIden, SearchQuery,
};
use crate::errors::SearchErrors;

use super::search::{apply_search, apply_search_filter};
use super::{device_query, media_query};

pub fn select_all_medias_by_device_id_order_by_date(
    user_id: &Uuid,
    device_id: &Uuid,
    search: MediaSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = media_query::select_all(user_id);

    join_media_session_by_device_id(&mut select, device_id);
    add_order_by_start_datetime(&mut select);
    // TODO Distinct?

    apply_search(select, search)
}

pub fn count_all_medias_by_device_id_order_by_date(
    user_id: &Uuid,
    device_id: &Uuid,
    search: MediaSearch,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = media_query::count_all(user_id);

    join_media_session_by_device_id(&mut select, device_id);
    // TODO Distinct?

    apply_search_filter(select, search)
}

pub fn select_all_devices_by_media_id_order_by_date(
    user_id: &Uuid,
    media_id: &Uuid,
    search: DeviceSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = device_query::select_all(user_id);

    join_media_session_by_media_id(&mut select, media_id);
    add_order_by_start_datetime(&mut select);
    // TODO Distinct?

    apply_search(select, search)
}

pub fn count_all_devices_by_media_id_order_by_date(
    user_id: &Uuid,
    media_id: &Uuid,
    search: DeviceSearch,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = device_query::count_all(user_id);

    join_media_session_by_media_id(&mut select, media_id);
    // TODO Distinct?

    apply_search_filter(select, search)
}

fn join_media_session_by_device_id(select: &mut SelectStatement, device_id: &Uuid) {
    select
        .left_join(
            MediaSessionIden::Table,
            Expr::col((MediaIden::Table, MediaIden::Id))
                .equals((MediaSessionIden::Table, MediaSessionIden::MediaId)),
        )
        .and_where(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::DeviceId))
                .eq(crate::uuid_utils::to_string(device_id)),
        );
}

fn join_media_session_by_media_id(select: &mut SelectStatement, media_id: &Uuid) {
    select
        .left_join(
            MediaSessionIden::Table,
            Expr::col((DeviceIden::Table, DeviceIden::UserId))
                .equals((MediaSessionIden::Table, MediaSessionIden::UserId))
                .and(
                    Expr::col((DeviceIden::Table, DeviceIden::Id))
                        .equals((MediaSessionIden::Table, MediaSessionIden::DeviceId)),
                ),
        )
        .and_where(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId))
                .eq(crate::uuid_utils::to_string(media_id)),
        );
}

fn add_order_by_start_datetime(select: &mut SelectStatement) {
    select.order_by(
        (MediaSessionIden::Table, MediaSessionIden::StartDate),
        Order::Asc,
    );
}
