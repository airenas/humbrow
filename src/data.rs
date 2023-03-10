use tokio::sync::Mutex;

pub struct Service {
    pub user_agent: String,
    pub exec_guard: Mutex<u64>,
    pub cookie_script: String,
    pub python: String,
}
