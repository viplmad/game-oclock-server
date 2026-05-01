use sea_query::{Alias, Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};
use uuid::Uuid;

use crate::entities::{
    AggregateQuery, ExternalMedia, ExternalMediaIden, Media, MediaAggregateSearch, MediaIden,
    MediaListSearch, MediaState, MediaStateIden, STATE_ADDED_DATETIME_ALIAS, STATE_NOTES_ALIAS,
    STATE_RATING_ALIAS, STATE_STATUS_ALIAS, STATE_UPDATED_DATETIME_ALIAS, SearchQuery,
};
use crate::errors::SearchErrors;

use super::search::{apply_aggregate_search, apply_search};

#[cfg(test)]
mod tests {
    use sea_query::PostgresQueryBuilder;
    use uuid::Uuid;

    use super::*;
    use crate::entities::{AggregateCountMetric, AggregateMetric, ColIden, FieldIden, FieldType};

    #[test]
    fn select_all_medias() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = select_all_with_search(
            &user_id,
            MediaListSearch {
                filter: None,
                sort: None,
                page: None,
                size: None,
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT "Media"."id", "Media"."kind", "Media"."title", "Media"."edition", "Media"."release_date", "Media"."genres", "Media"."series", "Media"."image_url", "Media"."parent_id", "Media"."parent_order", "Media"."added_datetime", "Media"."updated_datetime","#,
                r#""MediaState"."user_id", "MediaState"."status" AS "state_status", "MediaState"."rating" AS "state_rating", "MediaState"."notes" AS "state_notes", "MediaState"."added_datetime" AS "state_added_datetime", "MediaState"."updated_datetime" AS "state_updated_datetime","#,
                r#""ExternalMedia"."external_source", "ExternalMedia"."external_id""#,
                r#"FROM "Media""#,
                r#"LEFT JOIN "MediaState" ON "Media"."id" = "MediaState"."media_id""#,
                r#"LEFT JOIN "ExternalMedia" ON "Media"."id" = "ExternalMedia"."media_id" AND "ExternalMedia"."primary" = TRUE"#,
                r#"WHERE "MediaState"."user_id" = '00000000-0000-0000-0000-000000000000'"#,
                r#"LIMIT 500 OFFSET 0"#
            ].join(" ")
        );
    }

    #[test]
    fn count_all_medias() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_all_with_search(
            &user_id,
            MediaAggregateSearch {
                filter: None,
                aggr: AggregateMetric::Count(AggregateCountMetric::new(
                    FieldIden::Col(ColIden::new(MediaIden::Id, FieldType::String)),
                    None,
                    false,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT COUNT("Media"."id")"#,
                r#"FROM "Media""#,
                r#"LEFT JOIN "MediaState" ON "Media"."id" = "MediaState"."media_id""#,
                r#"WHERE "MediaState"."user_id" = '00000000-0000-0000-0000-000000000000'"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn select() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let external_ids = vec![
            (String::from("source1"), String::from("id1")),
            (String::from("source1"), String::from("id2")),
        ];
        let query = select_all_state_by_externals(&user_id, &external_ids);
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT "MediaState"."user_id", "MediaState"."media_id", "MediaState"."status", "MediaState"."rating", "MediaState"."notes", "MediaState"."added_datetime", "MediaState"."updated_datetime","#,
                r#""ExternalMedia"."external_source", "ExternalMedia"."external_id""#,
                r#"FROM "MediaState""#,
                r#"LEFT JOIN "ExternalMedia" ON "MediaState"."media_id" = "ExternalMedia"."media_id" AND "ExternalMedia"."primary" = TRUE"#,
                r#"WHERE "MediaState"."user_id" = '00000000-0000-0000-0000-000000000000'"#,
                r#"AND (("ExternalMedia"."external_source" = 'source1' AND "ExternalMedia"."external_id" = 'id1') OR ("ExternalMedia"."external_source" = 'source1' AND "ExternalMedia"."external_id" = 'id2'))"#,
            ].join(" ")
        );
    }
}

pub fn select_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_id(&mut select, id);
    add_fields(&mut select);

    join_state(&mut select, user_id);
    add_state_join_fields(&mut select);
    join_external(&mut select);
    add_external_join_fields(&mut select);

    select
}

pub fn select_data_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_state_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_state(&mut select);
    where_id_state(&mut select, user_id, id);
    add_state_fields(&mut select);

    select
}

pub fn select_all_state_by_externals(
    user_id: &Uuid,
    external_ids: &Vec<(String, String)>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_state(&mut select);
    add_state_fields(&mut select);
    select.and_where(
        Expr::col((MediaStateIden::Table, MediaStateIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );

    let mut ors = sea_query::Cond::any();
    for (source, id) in external_ids {
        ors = ors.add(
            Expr::col((ExternalMediaIden::Table, ExternalMediaIden::ExternalSource))
                .eq(String::from(source))
                .and(
                    Expr::col((ExternalMediaIden::Table, ExternalMediaIden::ExternalId))
                        .eq(String::from(id)),
                ),
        );
    }
    if !ors.is_empty() {
        select.cond_where(ors);
    }

    select.left_join(
        ExternalMediaIden::Table,
        Expr::col((MediaStateIden::Table, MediaStateIden::MediaId))
            .equals((ExternalMediaIden::Table, ExternalMediaIden::MediaId))
            .and(Expr::col((ExternalMediaIden::Table, ExternalMediaIden::Primary)).eq(true)),
    );
    add_external_join_fields(&mut select);

    select
}

pub fn select_by_external_id(user_id: &Uuid, source: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_external_id(&mut select, source, id);
    add_fields(&mut select);

    join_state(&mut select, user_id);
    add_state_join_fields(&mut select);

    select
}

pub fn select_data_by_external_id(source: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_external_id(&mut select, source, id);
    add_fields(&mut select);

    select
}

pub fn select_primary_external_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_external(&mut select);
    select
        .and_where(
            Expr::col((ExternalMediaIden::Table, ExternalMediaIden::MediaId))
                .eq(crate::uuid_utils::to_string(id)),
        )
        .and_where(Expr::col((ExternalMediaIden::Table, ExternalMediaIden::Primary)).eq(true));
    select
        .column((ExternalMediaIden::Table, ExternalMediaIden::MediaId))
        .column((ExternalMediaIden::Table, ExternalMediaIden::ExternalSource))
        .column((ExternalMediaIden::Table, ExternalMediaIden::ExternalId))
        .column((ExternalMediaIden::Table, ExternalMediaIden::Primary));

    select
}

pub fn select_all_by_parent_id(user_id: &Uuid, parent_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_parent_id(&mut select, parent_id);
    add_fields(&mut select);

    join_state(&mut select, user_id);
    add_state_join_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: &Uuid,
    search: MediaListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub fn aggregate_all_with_search(
    user_id: &Uuid,
    search: MediaAggregateSearch,
) -> Result<AggregateQuery, SearchErrors> {
    let select = aggregate_all(user_id);

    apply_aggregate_search(select, search)
}

pub(super) fn select_all(user_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from(&mut select);
    add_fields(&mut select);

    join_state(&mut select, user_id);
    add_state_join_fields(&mut select);
    join_external(&mut select);
    add_external_join_fields(&mut select);

    select
}

pub(super) fn aggregate_all(user_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from(&mut select);
    join_state(&mut select, user_id);

    select
}

pub fn insert_data(media: &Media) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(MediaIden::Table)
        .columns([
            MediaIden::Id,
            MediaIden::Kind,
            MediaIden::Title,
            MediaIden::Edition,
            MediaIden::ReleaseDate,
            MediaIden::Genres,
            MediaIden::Series,
            MediaIden::ImageUrl,
            MediaIden::ParentId,
            MediaIden::ParentOrder,
            MediaIden::AddedDatetime,
            MediaIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&media.id).into(),
            media.kind.clone().into(),
            media.title.clone().into(),
            media.edition.clone().into(),
            media.release_date.into(),
            format!("{{{}}}", media.genres.join(",")).into(),
            format!("{{{}}}", media.series.join(",")).into(),
            media.image_url.clone().into(),
            media
                .parent_id
                .map(|v| crate::uuid_utils::to_string(&v))
                .into(),
            media.parent_order.into(),
            media.added_datetime.into(),
            media.updated_datetime.into(),
        ]);

    insert
}

pub fn insert_state(media: &MediaState) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(MediaStateIden::Table)
        .columns([
            MediaStateIden::UserId,
            MediaStateIden::MediaId,
            MediaStateIden::Status,
            MediaStateIden::Rating,
            MediaStateIden::Notes,
            MediaStateIden::AddedDatetime,
            MediaStateIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&media.user_id).into(),
            crate::uuid_utils::to_string(&media.media_id).into(),
            media.status.into(),
            media.rating.into(),
            media.notes.clone().into(),
            media.added_datetime.into(),
            media.updated_datetime.into(),
        ]);

    insert
}

pub fn insert_external(media: &ExternalMedia) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(ExternalMediaIden::Table)
        .columns([
            ExternalMediaIden::MediaId,
            ExternalMediaIden::ExternalSource,
            ExternalMediaIden::ExternalId,
            ExternalMediaIden::Primary,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&media.media_id).into(),
            media.external_source.clone().into(),
            media.external_id.clone().into(),
            media.primary.into(),
        ]);

    insert
}

pub fn update_data_by_id(media: &Media) -> impl QueryStatementWriter {
    update_data_values_by_id(
        &media.id,
        vec![
            (MediaIden::Kind, media.kind.clone().into()),
            (MediaIden::Title, media.title.clone().into()),
            (MediaIden::Edition, media.edition.clone().into()),
            (MediaIden::ReleaseDate, media.release_date.into()),
            (
                MediaIden::Genres,
                format!("{{{}}}", media.genres.join(",")).into(),
            ),
            (
                MediaIden::Series,
                format!("{{{}}}", media.series.join(",")).into(),
            ),
            (MediaIden::ImageUrl, media.image_url.clone().into()),
            (
                MediaIden::ParentId,
                media
                    .parent_id
                    .map(|v| crate::uuid_utils::to_string(&v))
                    .into(),
            ),
            (MediaIden::ParentOrder, media.parent_order.into()),
            (MediaIden::UpdatedDatetime, media.updated_datetime.into()),
        ],
    )
}

pub fn update_parent_id_by_id(id: &Uuid, parent_id: Option<Uuid>) -> impl QueryStatementWriter {
    update_data_values_by_id(
        id,
        vec![
            (
                MediaIden::ParentId,
                parent_id.map(|id| crate::uuid_utils::to_string(&id)).into(),
            ),
            (MediaIden::UpdatedDatetime, crate::date_utils::now().into()),
        ],
    )
}

fn update_data_values_by_id(
    id: &Uuid,
    values: Vec<(MediaIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(MediaIden::Table)
        .values(values)
        .and_where(Expr::col(MediaIden::Id).eq(crate::uuid_utils::to_string(id)));

    update
}

pub fn update_state_by_id(media: &MediaState) -> impl QueryStatementWriter {
    update_state_values_by_id(
        &media.user_id,
        &media.media_id,
        vec![
            (MediaStateIden::Status, media.status.into()),
            (MediaStateIden::Rating, media.rating.into()),
            (MediaStateIden::Notes, media.notes.clone().into()),
            (
                MediaStateIden::UpdatedDatetime,
                media.updated_datetime.into(),
            ),
        ],
    )
}

pub fn update_status_by_id(user_id: &Uuid, id: &Uuid, status: i16) -> impl QueryStatementWriter {
    update_state_values_by_id(
        user_id,
        id,
        vec![
            (MediaStateIden::Status, status.into()),
            (
                MediaStateIden::UpdatedDatetime,
                crate::date_utils::now().into(),
            ),
        ],
    )
}

fn update_state_values_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    values: Vec<(MediaStateIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(MediaStateIden::Table)
        .values(values)
        .and_where(Expr::col(MediaStateIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(MediaStateIden::MediaId).eq(crate::uuid_utils::to_string(media_id)));

    update
}

pub fn update_external_by_id(media: &ExternalMedia) -> impl QueryStatementWriter {
    update_external_values_by_id(
        &media.media_id,
        vec![
            (
                ExternalMediaIden::ExternalSource,
                media.external_source.clone().into(),
            ),
            (
                ExternalMediaIden::ExternalId,
                media.external_id.clone().into(),
            ),
            (ExternalMediaIden::Primary, media.primary.into()),
        ],
    )
}

fn update_external_values_by_id(
    media_id: &Uuid,
    values: Vec<(ExternalMediaIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(ExternalMediaIden::Table)
        .values(values)
        .and_where(
            Expr::col(ExternalMediaIden::MediaId).eq(crate::uuid_utils::to_string(media_id)),
        );

    update
}

pub fn delete_data_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(MediaIden::Table)
        .and_where(Expr::col(MediaIden::Id).eq(crate::uuid_utils::to_string(id)));

    delete
}

pub fn delete_state_by_id(user_id: &Uuid, media_id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(MediaStateIden::Table)
        .and_where(Expr::col(MediaStateIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(MediaStateIden::MediaId).eq(crate::uuid_utils::to_string(media_id)));

    delete
}

pub fn delete_external_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(ExternalMediaIden::Table)
        .and_where(Expr::col(ExternalMediaIden::MediaId).eq(crate::uuid_utils::to_string(id)));

    delete
}

pub fn exists_data_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_state_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_state(&mut select);
    where_id_state(&mut select, user_id, id);
    add_state_id_field(&mut select);

    select
}

pub fn exists_any_state_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_state(&mut select);
    select.and_where(
        Expr::col((MediaStateIden::Table, MediaStateIden::MediaId))
            .eq(crate::uuid_utils::to_string(id)),
    );
    add_state_id_field(&mut select);

    select
}

pub fn exists_any_by_parent_id(parent_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_parent_id(&mut select, parent_id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_title_and_edition(title: &str, edition: &str) -> SelectStatement {
    let mut select = Query::select();

    from(&mut select);
    add_id_field(&mut select);
    select
        .and_where(Expr::col(MediaIden::Title).eq(title))
        .and_where(Expr::col(MediaIden::Edition).eq(edition));

    select
}

pub fn exists_by_title_and_edition_and_id_not(
    title: &str,
    edition: &str,
    id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = exists_by_title_and_edition(title, edition);

    select.and_where(Expr::col(MediaIden::Id).ne(crate::uuid_utils::to_string(id)));

    select
}

pub fn exists_by_external_source_and_external_id_and_id_not(
    external_source: &str,
    external_id: &str,
    id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_external(&mut select);
    add_external_id_field(&mut select);
    select
        .and_where(Expr::col(ExternalMediaIden::ExternalSource).eq(external_source))
        .and_where(Expr::col(ExternalMediaIden::ExternalId).eq(external_id))
        .and_where(Expr::col((ExternalMediaIden::Table, ExternalMediaIden::Primary)).eq(true));

    select.and_where(Expr::col(ExternalMediaIden::MediaId).ne(crate::uuid_utils::to_string(id)));

    select
}

fn from(select: &mut SelectStatement) {
    select.from(MediaIden::Table);
}

fn from_state(select: &mut SelectStatement) {
    select.from(MediaStateIden::Table);
}

fn from_external(select: &mut SelectStatement) {
    select.from(ExternalMediaIden::Table);
}

fn where_id(select: &mut SelectStatement, id: &Uuid) {
    select.and_where(
        Expr::col((MediaIden::Table, MediaIden::Id)).eq(crate::uuid_utils::to_string(id)),
    );
}

fn where_id_state(select: &mut SelectStatement, user_id: &Uuid, id: &Uuid) {
    select
        .and_where(
            Expr::col((MediaStateIden::Table, MediaStateIden::MediaId))
                .eq(crate::uuid_utils::to_string(id)),
        )
        .and_where(
            Expr::col((MediaStateIden::Table, MediaStateIden::UserId))
                .eq(crate::uuid_utils::to_string(user_id)),
        );
}

fn where_external_id(select: &mut SelectStatement, source: &str, id: &str) {
    select
        .and_where(
            Expr::col((ExternalMediaIden::Table, ExternalMediaIden::ExternalSource))
                .eq(String::from(source)),
        )
        .and_where(
            Expr::col((ExternalMediaIden::Table, ExternalMediaIden::ExternalId))
                .eq(String::from(id)),
        );
}

fn where_parent_id(select: &mut SelectStatement, parent_id: &Uuid) {
    select.and_where(
        Expr::col((MediaIden::Table, MediaIden::ParentId))
            .eq(crate::uuid_utils::to_string(parent_id)),
    );
}

fn join_state(select: &mut SelectStatement, user_id: &Uuid) {
    select
        .left_join(
            MediaStateIden::Table,
            Expr::col((MediaIden::Table, MediaIden::Id))
                .equals((MediaStateIden::Table, MediaStateIden::MediaId)),
        )
        .and_where(
            Expr::col((MediaStateIden::Table, MediaStateIden::UserId))
                .eq(crate::uuid_utils::to_string(user_id)),
        );
}

fn join_external(select: &mut SelectStatement) {
    select.left_join(
        ExternalMediaIden::Table,
        Expr::col((MediaIden::Table, MediaIden::Id))
            .equals((ExternalMediaIden::Table, ExternalMediaIden::MediaId))
            .and(Expr::col((ExternalMediaIden::Table, ExternalMediaIden::Primary)).eq(true)),
    );
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((MediaIden::Table, MediaIden::Id));
}

fn add_state_id_field(select: &mut SelectStatement) {
    select.column((MediaStateIden::Table, MediaStateIden::MediaId));
}

fn add_external_id_field(select: &mut SelectStatement) {
    select.column((ExternalMediaIden::Table, ExternalMediaIden::MediaId));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((MediaIden::Table, MediaIden::Kind))
        .column((MediaIden::Table, MediaIden::Title))
        .column((MediaIden::Table, MediaIden::Edition))
        .column((MediaIden::Table, MediaIden::ReleaseDate))
        .column((MediaIden::Table, MediaIden::Genres))
        .column((MediaIden::Table, MediaIden::Series))
        .column((MediaIden::Table, MediaIden::ImageUrl))
        .column((MediaIden::Table, MediaIden::ParentId))
        .column((MediaIden::Table, MediaIden::ParentOrder))
        .column((MediaIden::Table, MediaIden::AddedDatetime))
        .column((MediaIden::Table, MediaIden::UpdatedDatetime));
}

fn add_external_join_fields(select: &mut SelectStatement) {
    select
        .column((ExternalMediaIden::Table, ExternalMediaIden::ExternalSource))
        .column((ExternalMediaIden::Table, ExternalMediaIden::ExternalId));
}

fn add_state_join_fields(select: &mut SelectStatement) {
    select
        .column((MediaStateIden::Table, MediaStateIden::UserId))
        .expr_as(
            Expr::col((MediaStateIden::Table, MediaStateIden::Status)),
            Alias::new(STATE_STATUS_ALIAS),
        )
        .expr_as(
            Expr::col((MediaStateIden::Table, MediaStateIden::Rating)),
            Alias::new(STATE_RATING_ALIAS),
        )
        .expr_as(
            Expr::col((MediaStateIden::Table, MediaStateIden::Notes)),
            Alias::new(STATE_NOTES_ALIAS),
        )
        .expr_as(
            Expr::col((MediaStateIden::Table, MediaStateIden::AddedDatetime)),
            Alias::new(STATE_ADDED_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((MediaStateIden::Table, MediaStateIden::UpdatedDatetime)),
            Alias::new(STATE_UPDATED_DATETIME_ALIAS),
        );
}

fn add_state_fields(select: &mut SelectStatement) {
    select
        .column((MediaStateIden::Table, MediaStateIden::UserId))
        .column((MediaStateIden::Table, MediaStateIden::MediaId))
        .column((MediaStateIden::Table, MediaStateIden::Status))
        .column((MediaStateIden::Table, MediaStateIden::Rating))
        .column((MediaStateIden::Table, MediaStateIden::Notes))
        .column((MediaStateIden::Table, MediaStateIden::AddedDatetime))
        .column((MediaStateIden::Table, MediaStateIden::UpdatedDatetime));
}
