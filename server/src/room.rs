use crate::game::*;

#[derive(Debug, Default)]
pub struct Room {
    pub game: Game,
    pub p1: Option<String>,
    pub p2: Option<String>,
}

impl Room {
    pub fn new() -> Self {
        Default::default()
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
