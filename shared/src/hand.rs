use std::fmt::Display;
use std::mem;

use crate::card::*;
use crate::deck::*;
use crate::error::*;

#[derive(Default, Debug, Clone)]
pub struct Hand {
    hand: Vec<Option<Card>>,
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

    pub fn play(&mut self, deck: &mut Deck, i: usize) -> Result<Card, GameError> {
        let card = self
            .hand
            .get_mut(i)
            .map(|card| mem::take(card))
            .ok_or(GameError::InvalidIndex(i))?
            .ok_or(GameError::NoCard(i));

        // we know that i is valid b/c of the guard in the definition for `card`
        self.hand[i] = deck.draw();

        card
    }
}
