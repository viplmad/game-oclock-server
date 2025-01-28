use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameGenreIden, GameIden, GenreIden};

use super::{game_query, genre_query};

pub fn select_all_games_by_genre_id(user_id: &str, genre_id: &str) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_genre_by_genre_id(&mut select, genre_id);

    select
}

pub fn select_all_genres_by_game_id(user_id: &str, game_id: &str) -> impl QueryStatementWriter {
    let mut select = genre_query::select_all(user_id);

    join_game_genre_by_game_id(&mut select, game_id);

    select
}

pub fn insert(user_id: &str, game_id: &str, genre_id: &str) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameGenreIden::Table)
        .columns([
            GameGenreIden::UserId,
            GameGenreIden::GameId,
            GameGenreIden::GenreId,
        ])
        .values_panic([user_id.into(), game_id.into(), genre_id.into()]);

    insert
}

pub fn delete_by_id(user_id: &str, game_id: &str, genre_id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameGenreIden::Table)
        .and_where(Expr::col(GameGenreIden::UserId).eq(user_id))
        .and_where(Expr::col(GameGenreIden::GameId).eq(game_id))
        .and_where(Expr::col(GameGenreIden::GenreId).eq(genre_id));

    delete
}

pub fn exists_by_id(user_id: &str, game_id: &str, genre_id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameGenreIden::Table, GameGenreIden::UserId))
        .and_where(Expr::col(GameGenreIden::GameId).eq(game_id))
        .and_where(Expr::col(GameGenreIden::GenreId).eq(genre_id));

    select
}

fn join_game_genre_by_genre_id(select: &mut SelectStatement, genre_id: &str) {
    select
        .left_join(
            GameGenreIden::Table,
            Expr::col((GameIden::Table, GameIden::UserId))
                .equals((GameGenreIden::Table, GameGenreIden::UserId))
                .and(
                    Expr::col((GameIden::Table, GameIden::Id))
                        .equals((GameGenreIden::Table, GameGenreIden::GameId)),
                ),
        )
        .and_where(Expr::col((GameGenreIden::Table, GameGenreIden::GenreId)).eq(genre_id));
}

fn join_game_genre_by_game_id(select: &mut SelectStatement, game_id: &str) {
    select
        .left_join(
            GameGenreIden::Table,
            Expr::col((GenreIden::Table, GenreIden::UserId))
                .equals((GameGenreIden::Table, GameGenreIden::UserId))
                .and(
                    Expr::col((GenreIden::Table, GenreIden::Id))
                        .equals((GameGenreIden::Table, GameGenreIden::GenreId)),
                ),
        )
        .and_where(Expr::col((GameGenreIden::Table, GameGenreIden::GameId)).eq(game_id));
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(GameGenreIden::Table)
        .and_where(Expr::col((GameGenreIden::Table, GameGenreIden::UserId)).eq(user_id));
}
