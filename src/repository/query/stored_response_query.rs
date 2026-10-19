use sea_query::{Expr, ExprTrait, OnConflict, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{StoredResponse, StoredResponseIden};

pub fn select_by_scope_and_request_hash(
    user_id: &Uuid,
    scope: &str,
    request_hash: &str,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_scope_and_request_hash(&mut select, scope, request_hash);
    add_fields(&mut select);

    select
}

pub fn update_last_used_date_by_scope_and_request_hash(
    user_id: &Uuid,
    scope: &str,
    request_hash: &str,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(StoredResponseIden::Table)
        .values(vec![
            (
                StoredResponseIden::LastUsedDate,
                crate::date_utils::now().into(),
            ),
            (
                StoredResponseIden::UpdatedDatetime,
                crate::date_utils::now().into(),
            ),
        ])
        .and_where(Expr::col(StoredResponseIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(StoredResponseIden::Scope).eq(scope))
        .and_where(Expr::col(StoredResponseIden::RequestHash).eq(request_hash));

    update
}

pub fn upsert(res: &StoredResponse) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(StoredResponseIden::Table)
        .columns([
            StoredResponseIden::UserId,
            StoredResponseIden::Scope,
            StoredResponseIden::RequestHash,
            StoredResponseIden::Response,
            StoredResponseIden::LastUsedDate,
            StoredResponseIden::AddedDatetime,
            StoredResponseIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&res.user_id).into(),
            res.scope.clone().into(),
            res.request_hash.clone().into(),
            res.response.clone().into(),
            res.last_used_date.into(),
            res.added_datetime.into(),
            res.updated_datetime.into(),
        ])
        .on_conflict(
            OnConflict::columns([
                StoredResponseIden::UserId,
                StoredResponseIden::Scope,
                StoredResponseIden::RequestHash,
            ])
            .update_columns([
                StoredResponseIden::Response,
                StoredResponseIden::UpdatedDatetime,
            ])
            .to_owned(),
        );

    insert
}

pub fn delete_by_scope_and_request_hash(
    user_id: &Uuid,
    scope: &str,
    request_hash: &str,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(StoredResponseIden::Table)
        .and_where(Expr::col(StoredResponseIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(StoredResponseIden::Scope).eq(scope))
        .and_where(Expr::col(StoredResponseIden::RequestHash).eq(request_hash));

    delete
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(StoredResponseIden::Table).and_where(
        Expr::col((StoredResponseIden::Table, StoredResponseIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_scope_and_request_hash(select: &mut SelectStatement, scope: &str, request_hash: &str) {
    select
        .and_where(Expr::col((StoredResponseIden::Table, StoredResponseIden::Scope)).eq(scope))
        .and_where(
            Expr::col((StoredResponseIden::Table, StoredResponseIden::RequestHash))
                .eq(request_hash),
        );
}

fn add_fields(select: &mut SelectStatement) {
    select
        .column((StoredResponseIden::Table, StoredResponseIden::UserId))
        .column((StoredResponseIden::Table, StoredResponseIden::Scope))
        .column((StoredResponseIden::Table, StoredResponseIden::RequestHash))
        .column((StoredResponseIden::Table, StoredResponseIden::Response))
        .column((StoredResponseIden::Table, StoredResponseIden::LastUsedDate))
        .column((StoredResponseIden::Table, StoredResponseIden::AddedDatetime))
        .column((
            StoredResponseIden::Table,
            StoredResponseIden::UpdatedDatetime,
        ));
}
