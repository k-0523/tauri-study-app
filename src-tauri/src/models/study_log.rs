#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct StudyLog {
    pub content: String,
    pub date: String,
}