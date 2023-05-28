use crate::deck::*;
use crate::hand::*;
use crate::pile::*;

#[derive(Default, Debug, Clone)]
pub struct Game {
    pub deck: Deck,
    pub pile: Pile,
    pub hands: (Hand, Hand),
    pub face_downs: (Hand, Hand),
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
}
