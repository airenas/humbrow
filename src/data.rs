use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

pub struct Service {
    pub user_agent: String,
    pub exec_guard: Mutex<u64>,
    pub reap_guard: Arc<RwLock<i32>>,
    pub cookie_script: String,
    pub python: String,
}
