use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};
use uuid::Uuid;

use crate::entities::{Genre, GenreIden, GenreSearch, SearchQuery};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: &Uuid,
    search: GenreSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub(super) fn select_all(user_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);

    select
}

pub fn insert(genre: &Genre) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GenreIden::Table)
        .columns([
            GenreIden::Id,
            GenreIden::UserId,
            GenreIden::Name,
            GenreIden::AddedDatetime,
            GenreIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&genre.id).into(),
            crate::uuid_utils::to_string(&genre.user_id).into(),
            genre.name.clone().into(),
            genre.added_datetime.into(),
            genre.updated_datetime.into(),
        ]);

    insert
}

pub fn update_by_id(genre: &Genre) -> impl QueryStatementWriter {
    update_values_by_id(
        &genre.user_id,
        &genre.id,
        vec![
            (GenreIden::Name, genre.name.clone().into()),
            (GenreIden::UpdatedDatetime, genre.updated_datetime.into()),
        ],
    )
}

fn update_values_by_id(
    user_id: &Uuid,
    id: &Uuid,
    values: Vec<(GenreIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(GenreIden::Table)
        .values(values)
        .and_where(Expr::col(GenreIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GenreIden::Id).eq(crate::uuid_utils::to_string(id)));

    update
}

pub fn delete_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GenreIden::Table)
        .and_where(Expr::col(GenreIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GenreIden::Id).eq(crate::uuid_utils::to_string(id)));

    delete
}

pub fn exists_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_name(user_id: &Uuid, name: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select.and_where(Expr::col(GenreIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(
    user_id: &Uuid,
    name: &str,
    id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(GenreIden::Id).ne(crate::uuid_utils::to_string(id)));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GenreIden::Table).and_where(
        Expr::col((GenreIden::Table, GenreIden::UserId)).eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_id(select: &mut SelectStatement, id: &Uuid) {
    select.and_where(
        Expr::col((GenreIden::Table, GenreIden::Id)).eq(crate::uuid_utils::to_string(id)),
    );
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((GenreIden::Table, GenreIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((GenreIden::Table, GenreIden::UserId))
        .column((GenreIden::Table, GenreIden::Name))
        .column((GenreIden::Table, GenreIden::AddedDatetime))
        .column((GenreIden::Table, GenreIden::UpdatedDatetime));
}
