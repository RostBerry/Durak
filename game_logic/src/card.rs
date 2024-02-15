use core::fmt;



#[derive(Debug)]
pub struct Card {
    value: u8 /*
    value looks like this:

         suit  trump buf  number
    0     11       0       1111
     */
}

pub enum CardCount {
    TwentyFour,
    ThirtySix,
    FiftyTwo
}

impl Card {

    const NUMBER_MASK: u8 = 0b00001111; //all bits except the card number set to 0
    const NUMBER_MASK_NEGATIVE: u8 = 0b11110000; //all bits except the card number set to 1
    const SUIT_MASK: u8 = 0b01100000; //all bits except the card suit set to 0
    const SUIT_MASK_NEGATIVE: u8 = 0b10011111; //all bits except the card suit set to 1

    const SUITS: [&'static str; 4] = ["clubs", "spades", "hearts", "diamonds"];
    const NAMES: [&'static str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

    fn clear_number(&mut self) {
        self.value &= Self::NUMBER_MASK_NEGATIVE;
    }

    fn clear_suit(&mut self) {
        self.value &= Self::SUIT_MASK_NEGATIVE;
    }

    pub fn num_to_suit(number: &u8) -> u8 {
        number << 5
    }

    pub fn bool_to_trump(is_trump: &bool) -> u8 {
        if *is_trump { 1 << 4 } else { 0 }
    }

    pub fn set_number(&mut self, number: u8) {
        self.clear_number();
        self.value |= number;
    }

    pub fn set_suit(&mut self, suit: u8) {
        self.clear_suit();
        self.value |= suit;
    }

    pub fn number(&self) -> u8 {
        self.value & Self::NUMBER_MASK
    }

    pub fn suit(&self) -> u8 {
        (self.value & Self::SUIT_MASK) >> 5
    }

    pub fn new() -> Card {
        Card {value: 0}
    }

    pub fn from_value(value: u8) -> Card {
        Card {value}
    }

    pub fn from_args(suit: &u8, number: &u8) -> Card {
        Card {value: Self::num_to_suit(suit) | Self::bool_to_trump(&false) | number}
    }

    pub fn to_suit_name(suit: u8) -> &'static str {
        Self::SUITS[suit as usize]
    }

    pub fn to_card_name(number: u8) -> &'static str {
        Self::NAMES[(number - 1) as usize]
    }

}

impl PartialEq for Card {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.value & Self::SUIT_MASK_NEGATIVE).cmp(&(other.value & Self::SUIT_MASK_NEGATIVE)))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.value & Self::SUIT_MASK_NEGATIVE).cmp(&(other.value & Self::SUIT_MASK_NEGATIVE))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", Self::to_suit_name(self.suit()), Self::to_card_name(self.number()))
    }
}