use crate::models::study_log::StudyLog;
use sqlx::SqlitePool;

pub async fn get(sqlite_pool: &SqlitePool, fetch_date: &str) -> Result<StudyLog, String> {
    let mut study_log = sqlx::query_as::<_, StudyLog>("select * from study_logs where date = ?")
        .bind(fetch_date)
        .fetch_one(sqlite_pool)
        .await
        .map_err(|e| e.to_string());

    study_log = match study_log {
        Ok(study_log) => Ok(study_log),
        Err(_) =>
        sqlx::query_as::<_, StudyLog>(
            r#"
                    INSERT INTO study_logs(content, date)
                    VALUES('', ?)
                    returning *
                "#,
            )
            .bind(fetch_date)
            .fetch_one(sqlite_pool)
            .await
            .map_err(|e| e.to_string())
    };
    study_log
}

// 関数の戻り値型
type DbResult<T> = Result<T, Box<dyn std::error::Error>>;
pub async fn save(sqlite_pool: &SqlitePool, post_content: &str, post_date: &str) ->DbResult<()> {
    // トランザクションを開始する
    let mut tx = sqlite_pool.begin().await?;
    sqlx::query(
        r#"
            UPDATE study_logs
            SET content = ?
            WHERE date = ?
        "#,
        )
        .bind(post_content)
        .bind(post_date)
        .execute(&mut tx)
        .await
        .expect("error");
    tx.commit().await?;

    Ok(())
}