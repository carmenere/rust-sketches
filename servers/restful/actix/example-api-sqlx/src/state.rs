use std::sync::Mutex;
use sqlx::postgres::PgPool;

use crate::settings::Setings;

pub struct AppData {
    pub settings: Setings,
    pub db: PgPool,
    pub req: Mutex<u64>,
}

impl AppData {
    pub async fn new() -> Self {
        let s = Setings::new();
        log::debug!("{}", &s.pg_url.to_string());
        Self {
            db: PgPool::connect(&s.pg_url.to_string()).await.unwrap(),
            req: Mutex::new(0),
            settings: s,
        }
    }
}
