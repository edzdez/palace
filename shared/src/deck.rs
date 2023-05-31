use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::*;

#[derive(Debug, Default, Clone)]
pub struct Deck {
    cards: Vec<Card>,
    curr_idx: usize,
}

impl Deck {
    pub fn new() -> Self {
        // I don't like this...
        let mut cards = Vec::new();
        for n in 1..=13 as u8 {
            cards.push(Card::new(Denomination::from(n), Suit::Spades));
            cards.push(Card::new(Denomination::from(n), Suit::Clubs));
            cards.push(Card::new(Denomination::from(n), Suit::Hearts));
            cards.push(Card::new(Denomination::from(n), Suit::Diamonds));
        }

        Self { cards, curr_idx: 0 }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
        self.curr_idx = 0;
    }

    pub fn clear(&mut self) {
        self.cards.clear();
        self.curr_idx = 0;
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.get(self.curr_idx).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.curr_idx >= self.cards.len()
    }
}
