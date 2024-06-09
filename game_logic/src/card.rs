use core::fmt;
use std::ops::Index;

use ansi_term::Color;



#[derive(Debug)]
pub struct Card {
    value: u8, /*
    value looks like this:

          suit  number
    00     11    1111
     */
    output_rows: [String; 11], // contains what user will see as representation of the card in terminal
}

pub enum CardCount {
    TwentyFour,
    ThirtySix,
    FiftyTwo
}

impl Card {

    const NUMBER_MASK: u8 = 0b00001111; //all bits except the card number set to 0
    const SUIT_MASK: u8 = 0b00110000; //all bits except the card suit set to 0
    const SUIT_MASK_NEGATIVE: u8 = 0b11001111; //all bits except the card suit set to 1

    const SUITS: [&'static str; 4] = ["clubs", "spades", "hearts", "diamonds"];
    const NAMES: [&'static str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

    const BIN_TO_SUIT: [&'static str; 4] = ["♣", "♠", "♥", "♦"];

    pub fn number(&self) -> u8 {
        self.value & Self::NUMBER_MASK
    }

    pub fn suit(&self) -> u8 {
        (self.value & Self::SUIT_MASK) >> 4
    }

    pub fn new() -> Card {
        let mut card: Card = Card {value: 0, output_rows: Default::default() };
        card.generate_output_rows();
        card
    }

    pub fn from_value(value: u8) -> Card {
        let mut card: Card = Card {value, output_rows: Default::default()};
        card.generate_output_rows();
        card
    }

    pub fn from_args(suit: u8, number: u8) -> Card {
        let mut card: Card = Card {value: suit << 4 | number, output_rows: Default::default()};
        card.generate_output_rows();
        card

    }

    pub fn to_suit_name(suit: u8) -> &'static str {
        Self::SUITS[suit as usize]
    }

    pub fn to_card_name(number: u8) -> &'static str {
        Self::NAMES[(number - 1) as usize]
    }

    fn generate_output_rows(&mut self) {
        let white = Color::White;
        let color = if self.suit() > 1 {Color::Red} else {Color::Blue};

        let suit_sym = Self::BIN_TO_SUIT[self.suit() as usize];
        let number = Self::NAMES[self.number() as usize - 1];
        let number_painted = format!("{}", color.paint(number));
        let dot_painted = format!("{}", white.paint("."));
        let suit_sym_painted = format!("{}", color.paint(suit_sym));
        self.output_rows[0] = format!("{}", white.paint(String::from("┌──────────────┐")));

        let more_than_three = format!("{}", if self.number() > 2 && self.number() < 10 {&suit_sym_painted} else {&dot_painted});
        let two_or_three = format!("{}", if self.number() > 0 && self.number() < 3 {&suit_sym_painted} else {&dot_painted});
        let seven_or_ten = format!("{}", if self.number() == 6 || self.number() == 9 {&suit_sym_painted} else {&dot_painted});
        let more_than_7 = format!("{}", if self.number() > 6 && self.number() < 10 {&suit_sym_painted} else {&dot_painted});
        let ten = format!("{}", if self.number() == 9 {&suit_sym_painted} else {&dot_painted});
        let center = format!("{}", if self.number() == 8 || self.number() == 2 || self.number() == 4 {&suit_sym_painted} else 
        { if self.number() > 9 {&number_painted} else {&dot_painted}});
        let six_or_seven = format!("{}", if self.number() == 5 || self.number() == 6 {&suit_sym_painted} else {&dot_painted});

        self.output_rows[1] = format!("│{}{}. . . . . . │", number_painted, if self.number() == 9 {""} else {" "});
        self.output_rows[2] = format!("│{} {} . {} . {} . │", suit_sym_painted, more_than_three, two_or_three, more_than_three);
        self.output_rows[3] = format!("│. . . {} . . . │", seven_or_ten);
        self.output_rows[4] = format!("│. {} . . . {} . │", more_than_7, more_than_7);
        self.output_rows[5] = format!("│. {} . {} . {} . │", six_or_seven, center, six_or_seven);
        self.output_rows[6] = format!("│. {} . . . {} . │", more_than_7, more_than_7);
        self.output_rows[7] = format!("│. . . {} . . . │", ten);
        self.output_rows[8] = format!("│. {} . {} . {} {} │", more_than_three, two_or_three, more_than_three, suit_sym_painted);
        self.output_rows[9] = format!("│. . . . . . {}{}│", number_painted, if self.number() == 9 {""} else {" "});
        self.output_rows[10] = String::from("└──────────────┘");

    }

}

// impl PartialEq for Card {
//     fn eq(&self, _other: &Self) -> bool {
//         false
//     }
// }

// impl Eq for Card {}

// impl PartialOrd for Card {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some((self.value & Self::SUIT_MASK_NEGATIVE).cmp(&(other.value & Self::SUIT_MASK_NEGATIVE)))
//     }
// }

// impl Ord for Card {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         (self.value & Self::SUIT_MASK_NEGATIVE).cmp(&(other.value & Self::SUIT_MASK_NEGATIVE))
//     }
// }

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", Self::to_suit_name(self.suit()), Self::to_card_name(self.number()))
    }
}

impl Index<usize> for Card {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.output_rows[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CARD_VALUE: u8 = 0b00101100;
    const SECOND_CARD_VALUE: u8 = 0b00111000;

    #[test]
    fn test_getters() {
        let card: Card = Card::from_value(TEST_CARD_VALUE);
        assert_eq!(card.number(), 12);
        assert_eq!(card.suit(), 2);
    }

    fn test_comparison() {
        let card: Card = Card::from_value(TEST_CARD_VALUE);
    }
}