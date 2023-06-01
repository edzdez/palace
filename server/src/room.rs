use tokio::sync::mpsc;

use crate::game::Game;
use crate::message::Message;

#[derive(Debug, Default)]
pub struct Room {
    pub game: Game,
    pub p1: Option<String>,
    pub p2: Option<String>,
    pub tx: Option<mpsc::Sender<Message>>,
}

impl Room {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init_game(&mut self) -> (mpsc::Sender<Message>, mpsc::Receiver<Message>) {
        let (tx, rx) = mpsc::channel(10);
        self.tx = Some(tx.clone());
        (tx, rx)
    }

    pub fn player_count(&self) -> u8 {
        if self.p1.is_none() {
            0
        } else if self.p2.is_none() {
            1
        } else {
            2
        }
    }
}
