use std::sync::{Arc, Mutex};

use diesel::PgConnection;

pub struct BotUser {
    pub name: String,
    pub id: String,
}

pub struct State {
    pub db: Arc<Mutex<PgConnection>>,
}
