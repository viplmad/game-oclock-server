use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};

use crate::entities::{Game, GameIden, GameSearch, GameUserInfoIden, SearchQuery};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    join_user_info(&mut select);
    add_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: &str,
    search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub(super) fn select_all(user_id: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    join_user_info(&mut select);
    add_fields(&mut select);

    select
}

pub(super) fn select_all_group_by_id(user_id: &str) -> SelectStatement {
    let mut select = select_all(user_id);

    select
        .group_by_col((GameIden::Table, GameIden::Id))
        .group_by_col((GameUserInfoIden::Table, GameUserInfoIden::UserId))
        .group_by_col((GameUserInfoIden::Table, GameUserInfoIden::GameId));

    select
}

pub fn insert(user_id: &str, id: &str, game: &Game) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameIden::Table)
        .columns([
            GameIden::UserId,
            GameIden::Id,
            GameIden::Name,
            GameIden::Edition,
            GameIden::ReleaseYear,
            GameIden::CoverFilename,
            GameIden::AddedDateTime,
            GameIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            id.into(),
            game.name.clone().into(),
            game.edition.clone().into(),
            game.release_year.into(),
            game.cover_filename.clone().into(),
            crate::date_utils::now().into(),
            crate::date_utils::now().into(),
        ]);

    insert
}

pub fn insert_user_info(user_id: &str, game_id: &str, game: &Game) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameUserInfoIden::Table)
        .columns([
            GameUserInfoIden::UserId,
            GameUserInfoIden::GameId,
            GameUserInfoIden::Status,
            GameUserInfoIden::Rating,
            GameUserInfoIden::Notes,
            GameUserInfoIden::SaveFolder,
            GameUserInfoIden::ScreenshotFolder,
            GameUserInfoIden::Backup,
            GameUserInfoIden::AddedDateTime,
            GameUserInfoIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            game_id.into(),
            game.status.into(),
            game.rating.into(),
            game.notes.clone().into(),
            game.save_folder.clone().into(),
            game.screenshot_folder.clone().into(),
            game.backup.into(),
            crate::date_utils::now().into(),
            crate::date_utils::now().into(),
        ]);

    insert
}

pub fn update_by_id(user_id: &str, id: &str, game: &Game) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![
            (GameIden::Name, game.name.clone().into()),
            (GameIden::Edition, game.edition.clone().into()),
            (GameIden::ReleaseYear, game.release_year.into()),
            (GameIden::CoverFilename, game.cover_filename.clone().into()),
        ],
    )
}

pub fn update_cover_filename_by_id(
    user_id: &str,
    id: &str,
    cover_filename: Option<String>,
) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![(GameIden::CoverFilename, cover_filename.into())],
    )
}

fn update_values_by_id(
    user_id: &str,
    id: &str,
    mut values: Vec<(GameIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    values.push((GameIden::UpdatedDateTime, crate::date_utils::now().into()));
    update
        .table(GameIden::Table)
        .values(values)
        .and_where(Expr::col(GameIden::UserId).eq(user_id))
        .and_where(Expr::col(GameIden::Id).eq(id));

    update
}

pub fn update_user_info_by_id(
    user_id: &str,
    game_id: &str,
    game: &Game,
) -> impl QueryStatementWriter {
    update_user_info_values_by_id(
        user_id,
        game_id,
        vec![
            (GameUserInfoIden::Status, game.status.into()),
            (GameUserInfoIden::Rating, game.rating.into()),
            (GameUserInfoIden::Notes, game.notes.clone().into()),
            (
                GameUserInfoIden::SaveFolder,
                game.save_folder.clone().into(),
            ),
            (
                GameUserInfoIden::ScreenshotFolder,
                game.screenshot_folder.clone().into(),
            ),
            (GameUserInfoIden::Backup, game.backup.into()),
        ],
    )
}

fn update_user_info_values_by_id(
    user_id: &str,
    game_id: &str,
    mut values: Vec<(GameUserInfoIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    values.push((
        GameUserInfoIden::UpdatedDateTime,
        crate::date_utils::now().into(),
    ));
    update
        .table(GameUserInfoIden::Table)
        .values(values)
        .and_where(Expr::col(GameUserInfoIden::UserId).eq(user_id))
        .and_where(Expr::col(GameUserInfoIden::GameId).eq(game_id));

    update
}

pub fn delete_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameIden::Table)
        .and_where(Expr::col(GameIden::UserId).eq(user_id))
        .and_where(Expr::col(GameIden::Id).eq(id));

    delete
}

pub fn delete_user_info_by_id(user_id: &str, game_id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameUserInfoIden::Table)
        .and_where(Expr::col(GameUserInfoIden::UserId).eq(user_id))
        .and_where(Expr::col(GameUserInfoIden::GameId).eq(game_id));

    delete
}

pub fn exists_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_name_and_edition(user_id: &str, name: &str, edition: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select
        .and_where(Expr::col(GameIden::Name).eq(name))
        .and_where(Expr::col(GameIden::Edition).eq(edition));

    select
}

pub fn exists_by_name_and_edition_and_id_not(
    user_id: &str,
    name: &str,
    edition: &str,
    id: &str,
) -> impl QueryStatementWriter {
    let mut select = exists_by_name_and_edition(user_id, name, edition);

    select.and_where(Expr::col(GameIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(GameIden::Table)
        .and_where(Expr::col((GameIden::Table, GameIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: &str) {
    select.and_where(Expr::col((GameIden::Table, GameIden::Id)).eq(id));
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
        .column((GameIden::Table, GameIden::Name))
        .column((GameIden::Table, GameIden::Edition))
        .column((GameIden::Table, GameIden::ReleaseYear))
        .column((GameIden::Table, GameIden::CoverFilename))
        .column((GameIden::Table, GameIden::AddedDateTime))
        .column((GameIden::Table, GameIden::UpdatedDateTime))
        .column((GameUserInfoIden::Table, GameUserInfoIden::Status))
        .column((GameUserInfoIden::Table, GameUserInfoIden::Rating))
        .column((GameUserInfoIden::Table, GameUserInfoIden::Notes))
        .column((GameUserInfoIden::Table, GameUserInfoIden::SaveFolder))
        .column((GameUserInfoIden::Table, GameUserInfoIden::ScreenshotFolder))
        .column((GameUserInfoIden::Table, GameUserInfoIden::Backup));
}
