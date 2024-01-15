use sqlx::mysql::MySqlPool;

#[derive(Debug)]
pub struct AppState {
    pub db: MySqlPool,
}