use crate::models::game::*;

use std::sync::Arc;
use tokio::sync::Mutex;

pub type DB = Arc<Mutex<Vec<Game>>>;

pub async fn create_db() -> DB {
    let db = Arc::new(Mutex::new(Vec::new()));
    db
}
