use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{GameGenre, GameGenreIden, GameIden, GenreIden};

use super::{game_query, genre_query};

pub fn select_all_games_by_genre_id(user_id: &Uuid, genre_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_genre_by_genre_id(&mut select, genre_id);

    select
}

pub fn select_all_genres_by_game_id(user_id: &Uuid, game_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = genre_query::select_all(user_id);

    join_game_genre_by_game_id(&mut select, game_id);

    select
}

pub fn insert(game_genre: &GameGenre) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameGenreIden::Table)
        .columns([
            GameGenreIden::UserId,
            GameGenreIden::GameId,
            GameGenreIden::GenreId,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game_genre.user_id).into(),
            crate::uuid_utils::to_string(&game_genre.game_id).into(),
            crate::uuid_utils::to_string(&game_genre.genre_id).into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: &Uuid, game_id: &Uuid, genre_id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameGenreIden::Table)
        .and_where(Expr::col(GameGenreIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameGenreIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameGenreIden::GenreId).eq(crate::uuid_utils::to_string(genre_id)));

    delete
}

pub fn exists_by_id(user_id: &Uuid, game_id: &Uuid, genre_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameGenreIden::Table, GameGenreIden::UserId))
        .and_where(Expr::col(GameGenreIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameGenreIden::GenreId).eq(crate::uuid_utils::to_string(genre_id)));

    select
}

fn join_game_genre_by_genre_id(select: &mut SelectStatement, genre_id: &Uuid) {
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
        .and_where(
            Expr::col((GameGenreIden::Table, GameGenreIden::GenreId))
                .eq(crate::uuid_utils::to_string(genre_id)),
        );
}

fn join_game_genre_by_game_id(select: &mut SelectStatement, game_id: &Uuid) {
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
        .and_where(
            Expr::col((GameGenreIden::Table, GameGenreIden::GameId))
                .eq(crate::uuid_utils::to_string(game_id)),
        );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GameGenreIden::Table).and_where(
        Expr::col((GameGenreIden::Table, GameGenreIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}
