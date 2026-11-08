use std::sync::{Arc, Mutex};

pub mod routes {
    pub mod debug;
    pub mod state;
    pub mod users;
}

#[derive(Clone)]
pub struct AppState {
    pub numbers: Arc<Mutex<Vec<i32>>>,
}
