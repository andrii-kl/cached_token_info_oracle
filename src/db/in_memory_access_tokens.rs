use lazy_static::lazy_static;
use rocket::yansi::Paint;
use tokio::sync::RwLock;
use std::collections::HashSet;

lazy_static! {
    pub static ref CASH: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}


/// - If the set did not previously contain this value, `true` is returned.
/// - If the set already contained this value, `false` is returned,
pub async fn insert(token: String) -> bool {
    let mut write_guard = CASH.write().await;
    write_guard.insert(token)
}

