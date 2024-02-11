

#[derive(Debug)]
pub struct Card {
    value: u8 /*
    value looks like this:

         suit  is_trump  number
    0     11       1      1111
     */
}

pub enum CardCount {
    TwentyFour,
    ThirtySix,
    FiftyTwo
}

impl Card {

    const NUMBER_MASK: u8 = 0b00001111; //all bits except the card number set to 0
    const NUMBER_MASK_NEGATIVE: u8 = 0b1111; //all bits except the card number set to 1
    const SUIT_MASK: u8 = 0b011; //all bits except the card suit set to 0
    const SUIT_MASK_NEGATIVE: u8 = 0b10011111; //all bits except the card suit set to 1
    const TRUMP_MASK: u8 = 0b0001; //all bits except the trump bit set to 0
    const TRUMP_MASK_NEGATIVE: u8 = 0b11101111; //all bits except the trump bit set to 1

    fn clear_number(&mut self) {
        self.value &= Self::NUMBER_MASK_NEGATIVE;
    }

    fn clear_trump(&mut self) {
        self.value &= Self::TRUMP_MASK_NEGATIVE;
    }

    fn clear_suit(&mut self) {
        self.value &= Self::SUIT_MASK_NEGATIVE;
    }

    pub fn num_to_suit(number: &u8) -> u8 {
        number << 5
    }

    pub fn num_to_trump(number: &u8) -> u8 {
        number << 4
    }

    pub fn set_number(&mut self, number: u8) {
        self.clear_number();
        self.value |= number;
    }

    pub fn set_trump(&mut self, number: u8) {
        self.clear_trump();
        self.value |= number;
    }

    pub fn set_suit(&mut self, suit: u8) {
        self.clear_suit();
        self.value |= suit;
    }

    pub fn number(&self) -> u8 {
        self.value & Self::NUMBER_MASK
    }

    pub fn trump(&self) -> u8 {
        self.value & Self::TRUMP_MASK
    }

    pub fn suit(&self) -> u8 {
        self.value & Self::SUIT_MASK
    }

    pub fn new() -> Card {
        Card {value: 0}
    }

    pub fn from_value(value: u8) -> Card {
        Card {value}
    }

    pub fn from_args(suit: &u8, is_trump: &u8, number: &u8) -> Card {
        Card {value: Self::num_to_suit(suit) | (is_trump << 4) | number}
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