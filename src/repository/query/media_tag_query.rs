use sea_query::{Alias, Expr, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{
    MediaIden, MediaSearch, MediaTag, MediaTagIden, SearchQuery, TAG_ADDED_DATETIME_ALIAS,
    TAG_ORDER_ALIAS, TAG_UPDATED_DATETIME_ALIAS, TagIden, TagSearch,
};
use crate::errors::SearchErrors;

use super::search::{apply_search, apply_search_filter};
use super::{media_query, tag_query};

#[cfg(test)]
mod tests {
    use sea_query::PostgresQueryBuilder;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn select_all_medias() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let tag_id = Uuid::try_parse("00000000-0000-0000-0000-000000000001").unwrap();
        let query = select_all_medias_by_tag_id(
            &user_id,
            &tag_id,
            MediaSearch {
                filter: None,
                sort: None,
                page: None,
                size: None,
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            "SELECT \"Media\".\"id\", \"Media\".\"kind\", \"Media\".\"title\", \"Media\".\"edition\", \"Media\".\"release_date\", \"Media\".\"genres\", \"Media\".\"series\", \"Media\".\"image_url\", \"Media\".\"parent_id\", \"Media\".\"parent_order\", \"Media\".\"added_datetime\", \"Media\".\"updated_datetime\", \
            \"MediaState\".\"user_id\", \"MediaState\".\"status\" AS \"state_status\", \"MediaState\".\"rating\" AS \"state_rating\", \"MediaState\".\"notes\" AS \"state_notes\", \"MediaState\".\"added_datetime\" AS \"state_added_datetime\", \"MediaState\".\"updated_datetime\" AS \"state_updated_datetime\", \
            \"MediaTag\".\"order\" AS \"tag_order\", \"MediaTag\".\"added_datetime\" AS \"tag_added_datetime\", \"MediaTag\".\"updated_datetime\" AS \"tag_updated_datetime\" \
            FROM \"Media\" \
            LEFT JOIN \"MediaState\" ON \"Media\".\"id\" = \"MediaState\".\"media_id\" \
            LEFT JOIN \"MediaTag\" ON \"Media\".\"id\" = \"MediaTag\".\"media_id\" \
            WHERE \"MediaState\".\"user_id\" = '00000000-0000-0000-0000-000000000000' \
            AND \"MediaTag\".\"tag_id\" = '00000000-0000-0000-0000-000000000001' \
            LIMIT 500 OFFSET 0"
        );
    }

    #[test]
    fn count_all_medias() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let tag_id = Uuid::try_parse("00000000-0000-0000-0000-000000000001").unwrap();
        let query = count_all_medias_by_tag_id(
            &user_id,
            &tag_id,
            MediaSearch {
                filter: None,
                sort: None,
                page: None,
                size: None,
            },
        );
        assert_eq!(
            query.unwrap().to_string(PostgresQueryBuilder),
            "SELECT COUNT(\"Media\".\"id\") \
            FROM \"Media\" \
            LEFT JOIN \"MediaState\" ON \"Media\".\"id\" = \"MediaState\".\"media_id\" \
            LEFT JOIN \"MediaTag\" ON \"Media\".\"id\" = \"MediaTag\".\"media_id\" \
            WHERE \"MediaState\".\"user_id\" = '00000000-0000-0000-0000-000000000000' \
            AND \"MediaTag\".\"tag_id\" = '00000000-0000-0000-0000-000000000001'"
        );
    }

    #[test]
    fn select_all_tags() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let media_id = Uuid::try_parse("00000000-0000-0000-0000-000000000001").unwrap();
        let query = select_all_tags_by_media_id(
            &user_id,
            &media_id,
            TagSearch {
                filter: None,
                sort: None,
                page: None,
                size: None,
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            "SELECT \"Tag\".\"id\", \"Tag\".\"user_id\", \"Tag\".\"name\", \"Tag\".\"added_datetime\", \"Tag\".\"updated_datetime\", \
            \"MediaTag\".\"order\" AS \"tag_order\", \"MediaTag\".\"added_datetime\" AS \"tag_added_datetime\", \"MediaTag\".\"updated_datetime\" AS \"tag_updated_datetime\" \
            FROM \"Tag\" \
            LEFT JOIN \"MediaTag\" ON \"Tag\".\"user_id\" = \"MediaTag\".\"user_id\" AND \"Tag\".\"id\" = \"MediaTag\".\"tag_id\" \
            WHERE \"Tag\".\"user_id\" = '00000000-0000-0000-0000-000000000000' \
            AND \"MediaTag\".\"media_id\" = '00000000-0000-0000-0000-000000000001' \
            LIMIT 500 OFFSET 0"
        );
    }

    #[test]
    fn count_all_tags() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let media_id = Uuid::try_parse("00000000-0000-0000-0000-000000000001").unwrap();
        let query = count_all_tags_by_media_id(
            &user_id,
            &media_id,
            TagSearch {
                filter: None,
                sort: None,
                page: None,
                size: None,
            },
        );
        assert_eq!(
            query.unwrap().to_string(PostgresQueryBuilder),
            "SELECT COUNT(\"Tag\".\"id\") \
            FROM \"Tag\" LEFT JOIN \"MediaTag\" ON \"Tag\".\"user_id\" = \"MediaTag\".\"user_id\" AND \"Tag\".\"id\" = \"MediaTag\".\"tag_id\" \
            WHERE \"Tag\".\"user_id\" = '00000000-0000-0000-0000-000000000000' \
            AND \"MediaTag\".\"media_id\" = '00000000-0000-0000-0000-000000000001'"
        );
    }
}

pub fn select_all_medias_by_tag_id(
    user_id: &Uuid,
    tag_id: &Uuid,
    search: MediaSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = media_query::select_all(user_id);

    join_media_tag_by_tag_id(&mut select, tag_id);
    add_fields(&mut select);

    apply_search(select, search)
}

pub fn count_all_medias_by_tag_id(
    user_id: &Uuid,
    tag_id: &Uuid,
    search: MediaSearch,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = media_query::count_all(user_id);

    join_media_tag_by_tag_id(&mut select, tag_id);

    apply_search_filter(select, search)
}

pub fn select_all_tags_by_media_id(
    user_id: &Uuid,
    media_id: &Uuid,
    search: TagSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = tag_query::select_all(user_id);

    join_media_tag_by_media_id(&mut select, media_id);
    add_fields(&mut select);

    apply_search(select, search)
}

pub fn count_all_tags_by_media_id(
    user_id: &Uuid,
    media_id: &Uuid,
    search: TagSearch,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = tag_query::count_all(user_id);

    join_media_tag_by_media_id(&mut select, media_id);

    apply_search_filter(select, search)
}

pub fn insert(media_tag: &MediaTag) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(MediaTagIden::Table)
        .columns([
            MediaTagIden::UserId,
            MediaTagIden::MediaId,
            MediaTagIden::TagId,
            MediaTagIden::Order,
            MediaTagIden::AddedDatetime,
            MediaTagIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&media_tag.user_id).into(),
            crate::uuid_utils::to_string(&media_tag.media_id).into(),
            crate::uuid_utils::to_string(&media_tag.tag_id).into(),
            media_tag.order.into(),
            media_tag.added_datetime.into(),
            media_tag.updated_datetime.into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: &Uuid, media_id: &Uuid, tag_id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(MediaTagIden::Table)
        .and_where(Expr::col(MediaTagIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(MediaTagIden::MediaId).eq(crate::uuid_utils::to_string(media_id)))
        .and_where(Expr::col(MediaTagIden::TagId).eq(crate::uuid_utils::to_string(tag_id)));

    delete
}

pub fn exists_by_id(user_id: &Uuid, media_id: &Uuid, tag_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((MediaTagIden::Table, MediaTagIden::UserId))
        .and_where(Expr::col(MediaTagIden::MediaId).eq(crate::uuid_utils::to_string(media_id)))
        .and_where(Expr::col(MediaTagIden::TagId).eq(crate::uuid_utils::to_string(tag_id)));

    select
}

fn join_media_tag_by_tag_id(select: &mut SelectStatement, tag_id: &Uuid) {
    select
        .left_join(
            MediaTagIden::Table,
            Expr::col((MediaIden::Table, MediaIden::Id))
                .equals((MediaTagIden::Table, MediaTagIden::MediaId)),
        )
        .and_where(
            Expr::col((MediaTagIden::Table, MediaTagIden::TagId))
                .eq(crate::uuid_utils::to_string(tag_id)),
        );
}

fn join_media_tag_by_media_id(select: &mut SelectStatement, media_id: &Uuid) {
    select
        .left_join(
            MediaTagIden::Table,
            Expr::col((TagIden::Table, TagIden::UserId))
                .equals((MediaTagIden::Table, MediaTagIden::UserId))
                .and(
                    Expr::col((TagIden::Table, TagIden::Id))
                        .equals((MediaTagIden::Table, MediaTagIden::TagId)),
                ),
        )
        .and_where(
            Expr::col((MediaTagIden::Table, MediaTagIden::MediaId))
                .eq(crate::uuid_utils::to_string(media_id)),
        );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(MediaTagIden::Table).and_where(
        Expr::col((MediaTagIden::Table, MediaTagIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn add_fields(select: &mut SelectStatement) {
    select
        .expr_as(
            Expr::col((MediaTagIden::Table, MediaTagIden::Order)),
            Alias::new(TAG_ORDER_ALIAS),
        )
        .expr_as(
            Expr::col((MediaTagIden::Table, MediaTagIden::AddedDatetime)),
            Alias::new(TAG_ADDED_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((MediaTagIden::Table, MediaTagIden::UpdatedDatetime)),
            Alias::new(TAG_UPDATED_DATETIME_ALIAS),
        );
}
