use std::fmt::{Debug, Display};

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub enum Face {
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jack => write!(f, "Jack"),
            Self::Queen => write!(f, "Queen"),
            Self::King => write!(f, "King"),
            Self::Ace => write!(f, "Ace"),
        }
    }
}

impl From<u8> for Face {
    fn from(n: u8) -> Self {
        match n {
            1 => Self::Ace,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => panic!("invalid face denomination!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Denomination {
    Number(u8),
    Face(Face),
}

impl Display for Denomination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Face(face) => write!(f, "{}", face),
        }
    }
}

impl From<u8> for Denomination {
    fn from(n: u8) -> Self {
        if n >= 14 {
            panic!("invalid card denomination!")
        } else if n == 1 || n > 10 {
            Self::Face(Face::from(n))
        } else {
            Self::Number(n)
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spades => write!(f, "Spades"),
            Self::Clubs => write!(f, "Clubs"),
            Self::Hearts => write!(f, "Hearts"),
            Self::Diamonds => write!(f, "Diamonds"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub denom: Denomination,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.denom, self.suit)
    }
}

impl Card {
    pub fn new(denom: Denomination, suit: Suit) -> Self {
        Self { denom, suit }
    }
}

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
