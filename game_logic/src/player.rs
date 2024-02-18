use std::io;

use ansi_term::Color;

use crate::{card::Card, game_manager::Action};

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

    pub fn get_move_as_third(&mut self) -> Action {
        self.print_deck();
        println!("Type \"attack <card index>\"/\"check\"/\"resign\"");
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_ascii_lowercase();
    
            if input == "check" {
                return Action::Check;
            } 
            if input == "resign" {
                return Action::Resign;
            }
            if input.starts_with("attack") {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("Invalid input, please try again.");
                    continue;
                }
                let index = match parts[1].parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Invalid input, please enter a valid card index.");
                        continue;
                    }
                };
                if index >= self.all_cards.len() {
                    println!("Invalid index, please enter a valid card index.");
                    continue;
                }
                return Action::Attack(self.all_cards.remove(index));
            }
            println!("Invalid input, please try again.");
        }
    }

    pub fn get_move_as_attacker(&mut self) -> Action {
        self.print_deck();
        println!("Type \"attack <card index>\"/\"resign\"");
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_ascii_lowercase();
            if input == "resign" {
                return Action::Resign;
            }
            if input.starts_with("attack") {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("Invalid input, please try again.");
                    continue;
                }
                let index = match parts[1].parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Invalid input, please enter a valid card index.");
                        continue;
                    }
                };
                if index >= self.all_cards.len() {
                    println!("Invalid index, please enter a valid card index.");
                    continue;
                }
                return Action::Attack(self.all_cards.remove(index));
            }
            println!("Invalid input, please try again.");
        }
    }

    pub fn get_move_as_defender(&mut self, can_be_defended: bool, can_be_transfered: bool) -> Action {
        self.print_deck();
        println!("Type \"respond <card index>\"/\"transfer <card index>\"/\"take\"/\"resign\"");
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_ascii_lowercase();
    
            if input == "take" {
                return Action::Take;
            } 
            if input == "resign" {
                return Action::Resign;
            }
            if input.starts_with("respond") {
                if !can_be_defended {
                    println!("You cannot respond to the attack now");
                    continue;
                }
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("Invalid input, please try again.");
                    continue;
                }
                let index = match parts[1].parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Invalid input, please enter a valid card index.");
                        continue;
                    }
                };
                if index >= self.all_cards.len() {
                    println!("Invalid index, please enter a valid card index.");
                    continue;
                }
                return Action::Defend(self.all_cards.remove(index));
            }
            if input.starts_with("transfer") {
                if !can_be_transfered {
                    println!("You cannot transfer card now");
                    continue;
                }
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("Invalid input, please try again.");
                    continue;
                }
                let index = match parts[1].parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Invalid input, please enter a valid card index.");
                        continue;
                    }
                };
                if index >= self.all_cards.len() {
                    println!("Invalid index, please enter a valid card index.");
                    continue;
                }
                return Action::Transfer(self.all_cards.remove(index));
            }
            println!("Invalid input, please try again.");
        }
    }

    pub fn print_deck(&self) {

        for i in 0..11 {
            for card in self.all_cards.iter() {
                print!("{} ", card[i]);
            }
            println!();
        }
        for i in 0..self.all_cards.len() {
            print!("{:^16} ", i);
        }
        println!();
    }
}