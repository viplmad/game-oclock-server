use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};
use uuid::Uuid;

use crate::entities::{SearchQuery, User, UserIden, UserSearch};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_by_username(username: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    add_fields(&mut select);
    select.and_where(Expr::col(UserIden::Username).eq(username));

    select
}

pub fn select_all_with_search(search: UserSearch) -> Result<SearchQuery, SearchErrors> {
    let select = select_all();

    apply_search(select, search)
}

pub(super) fn select_all() -> SelectStatement {
    let mut select = Query::select();

    from(&mut select);
    add_fields(&mut select);

    select
}

pub fn insert(user: &User) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(UserIden::Table)
        .columns([
            UserIden::Id,
            UserIden::Password,
            UserIden::Username,
            UserIden::AddedDatetime,
            UserIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&user.id).into(),
            user.password.clone().into(),
            user.username.clone().into(),
            user.added_datetime.into(),
            user.updated_datetime.into(),
        ]);

    insert
}

pub fn update_by_id(user: &User) -> impl QueryStatementWriter {
    update_values_by_id(
        &user.id,
        vec![
            (UserIden::Username, user.username.clone().into()),
            (UserIden::UpdatedDatetime, user.updated_datetime.into()),
        ],
    )
}

pub fn update_password_by_id(id: &Uuid, password: &str) -> impl QueryStatementWriter {
    update_values_by_id(
        id,
        vec![
            (UserIden::Password, password.into()),
            (UserIden::UpdatedDatetime, crate::date_utils::now().into()),
        ],
    )
}

pub fn update_admin_by_id(id: &Uuid, admin: bool) -> impl QueryStatementWriter {
    update_values_by_id(
        id,
        vec![
            (UserIden::Admin, admin.into()),
            (UserIden::UpdatedDatetime, crate::date_utils::now().into()),
        ],
    )
}

fn update_values_by_id(
    id: &Uuid,
    values: Vec<(UserIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(UserIden::Table)
        .values(values)
        .and_where(Expr::col(UserIden::Id).eq(crate::uuid_utils::to_string(id)));

    update
}

pub fn delete_by_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(crate::uuid_utils::to_string(id)));

    delete
}

pub fn exists_by_id(id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from(&mut select);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_username(username: &str) -> SelectStatement {
    let mut select = Query::select();

    from(&mut select);
    add_id_field(&mut select);
    select.and_where(Expr::col(UserIden::Username).eq(username));

    select
}

pub fn exists_by_username_and_id_not(username: &str, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = exists_by_username(username);

    where_id_not(&mut select, id);

    select
}

pub fn exists_by_admin_and_id_not(id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    add_id_field(&mut select);
    where_admin(&mut select, true);
    where_id_not(&mut select, id);

    select
}

pub fn exists_by_admin_and_id(id: &Uuid) -> impl QueryStatementWriter {
    let mut select = exists_by_id(id);

    where_admin(&mut select, true);

    select
}

pub fn exists_by_admin() -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_admin(&mut select, true);
    add_id_field(&mut select);

    select
}

fn from(select: &mut SelectStatement) {
    select.from(UserIden::Table);
}

fn where_id(select: &mut SelectStatement, id: &Uuid) {
    select
        .and_where(Expr::col((UserIden::Table, UserIden::Id)).eq(crate::uuid_utils::to_string(id)));
}

fn where_admin(select: &mut SelectStatement, admin: bool) {
    select.and_where(Expr::col((UserIden::Table, UserIden::Admin)).eq(admin));
}

fn where_id_not(select: &mut SelectStatement, id: &Uuid) {
    select
        .and_where(Expr::col((UserIden::Table, UserIden::Id)).ne(crate::uuid_utils::to_string(id)));
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((UserIden::Table, UserIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((UserIden::Table, UserIden::Username))
        .column((UserIden::Table, UserIden::Password))
        .column((UserIden::Table, UserIden::Admin))
        .column((UserIden::Table, UserIden::AddedDatetime))
        .column((UserIden::Table, UserIden::UpdatedDatetime));
}
