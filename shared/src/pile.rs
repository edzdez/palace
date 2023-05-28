use crate::card::*;

#[derive(Default, Debug, Clone)]
pub struct Pile {
    cards: Vec<Card>
}

impl Pile {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
