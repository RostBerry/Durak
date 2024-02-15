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

    // pub fn take_card_away(&mut self, card_to_remove: &Card) -> Card {
    //     let mut index = 0;
    //     while index < self.all_cards.len() {
    //         if *card_to_remove == self.all_cards[index] {
    //             return self.all_cards.remove(index);
    //         }
    //         index += 1;
    //     }
    //     panic!();
    // }

    pub fn get_move(&mut self) -> Card {
        self.print_deck();
        println!("Enter card: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let input = input.trim();

        let parse_result: Result<usize, _> = input.parse();

        let index: usize;

        match parse_result {
            Ok(value) => {
                index = value;
            }
            Err(_) => {
                println!("not good");
                panic!();
            }
        }
        self.all_cards.remove(index)
    }

    pub fn print_deck(&self) {
        for _card in self.all_cards.iter() {
            print!("-------------- ");
        }
        println!();
        for _card in self.all_cards.iter() {
            print!("|            | ");
        }
        println!();
        for _card in self.all_cards.iter() {
            print!("|            | ");
        }
        println!();
        for _card in self.all_cards.iter() {
            print!("|            | ");
        }
        println!();
        for card in self.all_cards.iter() {
            print!("|{:^12}| ", card.to_string());
        }
        println!();
        for _card in self.all_cards.iter() {
            print!("|            | ");
        }
        println!();
        for _card in self.all_cards.iter() {
            print!("|            | ");
        }
        println!();
        for _card in self.all_cards.iter() {
            print!("|            | ");
        }
        println!();
        for _card in self.all_cards.iter() {
            print!("-------------- ");
        }
        println!();
        let mut index = 0;
        while index < self.all_cards.len() {
            print!("{:^14} ", index);
            index += 1;
        }
        println!();
    }
}