use crate::{infrastructures::content, models::study_log::StudyLog};
use tauri::State;

#[tauri::command]
pub async fn get(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    date: &str
)-> Result<StudyLog, String> {
    let study_log = content::get(&*sqlite_pool, date).await.map_err(|e| e.to_string());
    study_log
}

#[tauri::command]
pub async fn save(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    content: &str,
    date: &str
)-> Result<(), String> {
    content::save(&*sqlite_pool, content, date)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}