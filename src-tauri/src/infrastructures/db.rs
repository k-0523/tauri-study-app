use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::str::FromStr;

// 関数の戻り値型
type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

pub async fn create_sqlite_pool() -> DbResult<SqlitePool> {
    let database_url = "remind-study.db";
    // コネクションの設定
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);
    // 上の設定を使ってコネクションプールを作成する
    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;
    Ok(sqlite_pool)
}