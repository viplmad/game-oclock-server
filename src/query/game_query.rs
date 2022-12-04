use chrono::Utc;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{Game, GameIden, GameSearch, GameUserInfoIden, SearchQuery};
use crate::errors::RepositoryError;

use super::search::apply_search;

pub fn select_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    join_user_info(&mut select);
    add_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: i32,
    search: GameSearch,
) -> Result<SearchQuery, RepositoryError> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub(super) fn select_all(user_id: i32) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    join_user_info(&mut select);
    add_fields(&mut select);

    select
}

pub fn insert(user_id: i32, game: &Game) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameIden::Table)
        .columns([
            GameIden::UserId,
            GameIden::Name,
            GameIden::Edition,
            GameIden::ReleaseYear,
            GameIden::CoverFilename,
            GameIden::AddedDateTime,
            GameIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            game.name.clone().into(),
            game.edition.clone().into(),
            game.release_year.into(),
            game.cover_filename.clone().into(),
            Utc::now().naive_utc().into(),
            Utc::now().naive_utc().into(),
        ])
        .returning(Query::returning().columns([GameIden::Id]));

    insert
}

pub fn insert_user_info(user_id: i32, game_id: i32, game: &Game) -> impl QueryStatementWriter {
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
            Utc::now().naive_utc().into(),
            Utc::now().naive_utc().into(),
        ])
        .returning(
            Query::returning().columns([GameUserInfoIden::UserId, GameUserInfoIden::GameId]),
        );

    insert
}

pub fn update_by_id(user_id: i32, id: i32, game: &Game) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(GameIden::Table)
        .values(vec![
            (GameIden::Name, game.name.clone().into()),
            (GameIden::Edition, game.edition.clone().into()),
            (GameIden::ReleaseYear, game.release_year.into()),
            (GameIden::CoverFilename, game.cover_filename.clone().into()),
            (GameIden::UpdatedDateTime, Utc::now().naive_utc().into()),
        ])
        .and_where(Expr::col(GameIden::UserId).eq(user_id))
        .and_where(Expr::col(GameIden::Id).eq(id))
        .returning(Query::returning().columns([GameIden::Id]));

    update
}

pub fn update_user_info_by_id(
    user_id: i32,
    game_id: i32,
    game: &Game,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(GameUserInfoIden::Table)
        .values(vec![
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
            (
                GameUserInfoIden::UpdatedDateTime,
                Utc::now().naive_utc().into(),
            ),
        ])
        .and_where(Expr::col(GameUserInfoIden::UserId).eq(user_id))
        .and_where(Expr::col(GameUserInfoIden::GameId).eq(game_id))
        .returning(
            Query::returning().columns([GameUserInfoIden::UserId, GameUserInfoIden::GameId]),
        );

    update
}

pub fn delete_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameIden::Table)
        .and_where(Expr::col(GameIden::UserId).eq(user_id))
        .and_where(Expr::col(GameIden::Id).eq(id));

    delete
}

pub fn delete_user_info_by_id(user_id: i32, game_id: i32) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameUserInfoIden::Table)
        .and_where(Expr::col(GameUserInfoIden::UserId).eq(user_id))
        .and_where(Expr::col(GameUserInfoIden::GameId).eq(game_id));

    delete
}

pub fn exists_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_name_and_edition(
    user_id: i32,
    name: &str,
    edition: &str,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select
        .and_where(Expr::col(GameIden::Name).eq(name))
        .and_where(Expr::col(GameIden::Edition).eq(edition));

    select
}

// TODO same as above but with notequal
pub fn exists_by_name_and_edition_and_id_not(
    user_id: i32,
    name: &str,
    edition: &str,
    id: i32,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select
        .and_where(Expr::col(GameIden::Name).eq(name))
        .and_where(Expr::col(GameIden::Edition).eq(edition))
        .and_where(Expr::col(GameIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(GameIden::Table)
        .and_where(Expr::col((GameIden::Table, GameIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: i32) {
    select.and_where(Expr::col((GameIden::Table, GameIden::Id)).eq(id));
}

fn join_user_info(select: &mut SelectStatement) {
    select.left_join(
        GameUserInfoIden::Table,
        Expr::tbl(GameIden::Table, GameIden::UserId)
            .equals(GameUserInfoIden::Table, GameUserInfoIden::UserId)
            .and(
                Expr::tbl(GameIden::Table, GameIden::Id)
                    .equals(GameUserInfoIden::Table, GameUserInfoIden::GameId),
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
