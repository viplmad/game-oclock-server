use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};
use uuid::Uuid;

use crate::entities::{Game, GameIden, GameSearch, GameUserInfo, GameUserInfoIden, SearchQuery};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    join_user_info(&mut select);
    add_fields(&mut select);

    select
}

pub fn select_all_by_base_game_id(
    user_id: &Uuid,
    base_game_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);
    select
        .and_where(Expr::col(GameIden::BaseGameId).eq(crate::uuid_utils::to_string(base_game_id)));

    select
}

pub fn select_all_with_search(
    user_id: &Uuid,
    search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub(super) fn select_all(user_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    join_user_info(&mut select);
    add_fields(&mut select);

    select
}

pub(super) fn select_all_group_by_id(user_id: &Uuid) -> SelectStatement {
    let mut select = select_all(user_id);

    select
        .group_by_col((GameIden::Table, GameIden::Id))
        .group_by_col((GameUserInfoIden::Table, GameUserInfoIden::UserId))
        .group_by_col((GameUserInfoIden::Table, GameUserInfoIden::GameId));

    select
}

pub fn insert(game: &Game) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameIden::Table)
        .columns([
            GameIden::Id,
            GameIden::UserId,
            GameIden::Title,
            GameIden::Edition,
            GameIden::ReleaseDate,
            GameIden::CoverUrl,
            GameIden::AddedDatetime,
            GameIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game.id).into(),
            crate::uuid_utils::to_string(&game.user_id).into(),
            game.title.clone().into(),
            game.edition.clone().into(),
            game.release_date.into(),
            game.cover_url.clone().into(),
            game.added_datetime.into(),
            game.updated_datetime.into(),
        ]);

    insert
}

pub fn insert_user_info(game: &GameUserInfo) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameUserInfoIden::Table)
        .columns([
            GameUserInfoIden::UserId,
            GameUserInfoIden::GameId,
            GameUserInfoIden::Status,
            GameUserInfoIden::Rating,
            GameUserInfoIden::Notes,
            GameUserInfoIden::AddedDatetime,
            GameUserInfoIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game.user_id).into(),
            crate::uuid_utils::to_string(&game.game_id).into(),
            game.status.into(),
            game.rating.into(),
            game.notes.clone().into(),
            game.added_datetime.into(),
            game.updated_datetime.into(),
        ]);

    insert
}

pub fn update_by_id(game: &Game) -> impl QueryStatementWriter {
    update_values_by_id(
        &game.user_id,
        &game.id,
        vec![
            (GameIden::Title, game.title.clone().into()),
            (GameIden::Edition, game.edition.clone().into()),
            (GameIden::ReleaseDate, game.release_date.into()),
            (GameIden::CoverUrl, game.cover_url.clone().into()),
            (GameIden::UpdatedDatetime, game.updated_datetime.into()),
        ],
    )
}

pub fn update_base_game_id_by_id(
    user_id: &Uuid,
    id: &Uuid,
    base_game_id: Option<Uuid>,
) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![
            (
                GameIden::BaseGameId,
                base_game_id
                    .map(|id| crate::uuid_utils::to_string(&id))
                    .into(),
            ),
            (GameIden::UpdatedDatetime, crate::date_utils::now().into()),
        ],
    )
}

fn update_values_by_id(
    user_id: &Uuid,
    id: &Uuid,
    values: Vec<(GameIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(GameIden::Table)
        .values(values)
        .and_where(Expr::col(GameIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameIden::Id).eq(crate::uuid_utils::to_string(id)));

    update
}

pub fn update_user_info_by_id(game: &GameUserInfo) -> impl QueryStatementWriter {
    update_user_info_values_by_id(
        &game.user_id,
        &game.game_id,
        vec![
            (GameUserInfoIden::Status, game.status.into()),
            (GameUserInfoIden::Rating, game.rating.into()),
            (GameUserInfoIden::Notes, game.notes.clone().into()),
            (
                GameUserInfoIden::UpdatedDatetime,
                game.updated_datetime.into(),
            ),
        ],
    )
}

fn update_user_info_values_by_id(
    user_id: &Uuid,
    game_id: &Uuid,
    values: Vec<(GameUserInfoIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(GameUserInfoIden::Table)
        .values(values)
        .and_where(Expr::col(GameUserInfoIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameUserInfoIden::GameId).eq(crate::uuid_utils::to_string(game_id)));

    update
}

pub fn delete_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameIden::Table)
        .and_where(Expr::col(GameIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameIden::Id).eq(crate::uuid_utils::to_string(id)));

    delete
}

pub fn delete_user_info_by_id(user_id: &Uuid, game_id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameUserInfoIden::Table)
        .and_where(Expr::col(GameUserInfoIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameUserInfoIden::GameId).eq(crate::uuid_utils::to_string(game_id)));

    delete
}

pub fn exists_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_title_and_edition(user_id: &Uuid, name: &str, edition: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select
        .and_where(Expr::col(GameIden::Title).eq(name))
        .and_where(Expr::col(GameIden::Edition).eq(edition));

    select
}

pub fn exists_by_title_and_edition_and_id_not(
    user_id: &Uuid,
    name: &str,
    edition: &str,
    id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = exists_by_title_and_edition(user_id, name, edition);

    select.and_where(Expr::col(GameIden::Id).ne(crate::uuid_utils::to_string(id)));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GameIden::Table).and_where(
        Expr::col((GameIden::Table, GameIden::UserId)).eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_id(select: &mut SelectStatement, id: &Uuid) {
    select
        .and_where(Expr::col((GameIden::Table, GameIden::Id)).eq(crate::uuid_utils::to_string(id)));
}

fn join_user_info(select: &mut SelectStatement) {
    select.left_join(
        GameUserInfoIden::Table,
        Expr::col((GameIden::Table, GameIden::UserId))
            .equals((GameUserInfoIden::Table, GameUserInfoIden::UserId))
            .and(
                Expr::col((GameIden::Table, GameIden::Id))
                    .equals((GameUserInfoIden::Table, GameUserInfoIden::GameId)),
            ),
    );
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((GameIden::Table, GameIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((GameIden::Table, GameIden::UserId))
        .column((GameIden::Table, GameIden::Title))
        .column((GameIden::Table, GameIden::Edition))
        .column((GameIden::Table, GameIden::ReleaseDate))
        .column((GameIden::Table, GameIden::BaseGameId))
        .column((GameIden::Table, GameIden::CoverUrl))
        .column((GameIden::Table, GameIden::AddedDatetime))
        .column((GameIden::Table, GameIden::UpdatedDatetime))
        .column((GameUserInfoIden::Table, GameUserInfoIden::Status))
        .column((GameUserInfoIden::Table, GameUserInfoIden::Rating))
        .column((GameUserInfoIden::Table, GameUserInfoIden::Notes));
}
