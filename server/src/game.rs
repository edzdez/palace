use shared::deck::*;
use shared::hand::*;
use shared::pile::*;

#[derive(Default, Debug, Clone)]
pub struct Game {
    pub deck: Deck,
    pub pile: Pile,
    pub p1_hand: Hand,
    pub p2_hand: Hand,
    pub p1_face_downs: Hand,
    pub p2_face_downs: Hand,
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
}
