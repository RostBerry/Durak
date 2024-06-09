use std::fmt;
use crate::card::Card;

#[derive(Debug)]
pub enum CardPairHalf {
    Empty,
    Filled(Card)
}

#[derive(Debug)] 
pub struct CardPair {
    pub first: CardPairHalf,
    pub second: CardPairHalf
}

impl CardPair {
    pub fn new(first: Card) -> Self {
        Self {
            first: CardPairHalf::Filled(first),
            second: CardPairHalf::Empty
        }
    }

    pub fn fill(&mut self, second: Card) {
        self.second = CardPairHalf::Filled(second);
    }

    pub fn is_filled(&self) -> bool {
        matches!(self.first, CardPairHalf::Filled(_)) && matches!(self.second, CardPairHalf::Filled(_))
    }
}

impl fmt::Display for CardPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.first {
            CardPairHalf::Filled(card) => return write!(f, "{}", card),
            CardPairHalf::Empty => panic!("Empty first pair half")
        }
        
    }
}
