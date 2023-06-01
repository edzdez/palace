use tokio::sync::mpsc;

use std::collections::HashMap;

use crate::message::Message;
use crate::room::Room;

// use once_cell to create our global variable
use once_cell::sync::Lazy;

// use RwLock for thread-safe access to a variable
use tokio::{
    sync::{Mutex, RwLock},
    task::JoinHandle,
};

// create a global data store `ROOMS`
pub static ROOMS: Lazy<RwLock<HashMap<String, Room>>> = Lazy::new(|| RwLock::new(HashMap::new()));

// create a global data store 'TASKS'
pub static TASKS: Lazy<Mutex<Vec<(mpsc::Sender<Message>, JoinHandle<()>)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));
