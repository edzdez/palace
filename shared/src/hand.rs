use std::fmt::Display;

use crate::card::*;

#[derive(Default, Debug, Clone)]
pub struct Hand {
    hand: Vec<Option<Card>>
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.hand)
    }
}

impl Hand {
    pub fn new() -> Self {
        Default::default()
    }
}



