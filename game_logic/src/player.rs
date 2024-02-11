use std::io;

use crate::card::Card;

#[derive(Debug)]
pub struct Player {
    pub all_cards: Vec<Card>
}

impl Player {

    pub fn new() -> Player {
        Player{all_cards: Vec::new()}
    }

    pub fn give_card(&mut self, card: Card) {
        self.all_cards.push(card);
    }

    pub fn get_move(&mut self) -> &Card {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let input = input.trim();

        let parse_result: Result<usize, _> = input.parse();

        let index: usize;

        match parse_result {
            Ok(value) => {
                println!("good");
                index = value;
            }
            Err(_) => {
                println!("not good");
                index = usize::MAX;
            }
        }
        println!("Card suit: {}", self.all_cards[index].suit());
        &self.all_cards[index]
    }
}