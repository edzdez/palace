use std::collections::HashMap;

use crate::room::*;

// use once_cell to create our global variable
use once_cell::sync::Lazy;

// use RwLock for thread-safe access to a variable
use tokio::sync::RwLock;

// create a global data store `DATA`
pub static DATA: Lazy<RwLock<HashMap<String, Room>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
