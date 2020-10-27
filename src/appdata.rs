use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct AppData {
    pub pool: sqlx::postgres::PgPool,
    pub sessions: Arc<RwLock<HashMap<String, String>>>,
}
