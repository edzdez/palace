use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::card::*;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
    curr_idx: usize,
}

impl Default for Deck {
    fn default() -> Self {
        // I don't like this...
        let mut cards = Vec::new();
        for n in 1..=13 as u8 {
            cards.push(Card::new(Denomination::from(n), Suit::Spades));
            cards.push(Card::new(Denomination::from(n), Suit::Clubs));
            cards.push(Card::new(Denomination::from(n), Suit::Hearts));
            cards.push(Card::new(Denomination::from(n), Suit::Diamonds));
        }

        Self {
            cards,
            curr_idx: 0
        }
    }
}

impl Deck {
    fn new() -> Self {
        Default::default()
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
        self.curr_idx = 0;
    }

    fn deal(&mut self) -> Option<Card> {
        self.cards.get(self.curr_idx).cloned()
    }
}
