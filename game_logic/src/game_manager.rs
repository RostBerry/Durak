use crate::{card::{Card, CardCount}, player::Player};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub enum Action {
    Attack(Card),
    Defend(Card),
    Check,
    Transfer(Card),
    Resign,
    Take
}

pub enum ThirdAttackResult {
    Beat,
    Attacked 
}

pub struct GameManager {
    pub all_players: Vec<Player>,
    pub card_stack: Vec<Card>,
    pub card_count: CardCount,
    pub trump_suit: u8,
    pub is_first_beat: bool,
    player_index: i8

}

impl GameManager {

    pub fn new(mut player_count: u8, card_count: CardCount) -> GameManager {
        let mut all_players: Vec<Player> = Vec::new();
        while player_count > 0 {
            all_players.push(Player::new());
            player_count -= 1;
        }
        let card_stack: Vec<Card> = Self::new_card_stack(&card_count);
        let trump_suit;
        match card_stack.first() {
            Some(card) => trump_suit = card.suit(),
            None => panic!()
        }
        GameManager{all_players, card_stack, card_count, trump_suit, is_first_beat: false, player_index: 0}
    }

    pub fn run_game(&mut self) {        
        let mut iteration_count: u8 = 0;

        while iteration_count < 6 { //Cards giveaway
            for player in self.all_players.iter_mut() {
                if let Some(card) = self.card_stack.pop() {
                    player.give_card(card);
                }
            }
            iteration_count += 1;
        }
        
        match self.card_stack.first() { //Letting know the players about the last card in the deck
            Some(card) => {
                println!("\nLast card: {}\n", card);
                self.trump_suit = card.suit();
            },
            None => panic!()
        }

        loop { //game loop
            let mut cards_thrown: u8 = 0;
            let mut all_cards_thrown: Vec<Card> = Vec::new();
            let mut unresponded_attacks: Vec<Card> = Vec::new();
            let mut is_beaten: bool = false;
            let mut can_be_transfered = true;
            
            while !self.all_players[Self::clamp(self.player_index, self.all_players.len())].all_cards.is_empty() && //attacker has some cards
                !self.all_players[Self::clamp(self.player_index + 1, self.all_players.len())].all_cards.is_empty() && //defender has some cards
                !self.is_cards_limit_exceeded(&cards_thrown) {
                
                let attacker_index = Self::clamp(self.player_index, self.all_players.len());
                let defender_index = Self::clamp(self.player_index + 1, self.all_players.len());

                if cards_thrown == 0 { //first attack
                    println!("Player {} to move", attacker_index + 1);
                    match self.all_players[attacker_index].get_move_as_attacker() {
                        Action::Attack(attack) => {
                            println!("Player {} attacks with {}", attacker_index + 1, attack);
                            unresponded_attacks.push(attack);
                            cards_thrown += 1;
                            continue;
                        }
                        Action::Resign => {
                            println!("Player {} resigns", attacker_index + 1);
                            panic!(); //no resignation logic yet
                        }
                        _ => { // attacker shouldn't take, transfer, check or defend in the first attack
                            panic!();
                        }
                    }
                } else { // next player already defended himself once
                    match self.proccess_move_from_third(&mut unresponded_attacks, &attacker_index, 
                        &mut cards_thrown, &mut is_beaten) {
                        ThirdAttackResult::Attacked => {continue;}
                        ThirdAttackResult::Beat => {
                            if unresponded_attacks.is_empty() {
                                break; //going to the next iteration of game loop
                            }
                        } 
                    }
                }
                
                
                let mut unresponded_attack_index = 0;
                while unresponded_attack_index < unresponded_attacks.len() {
                    println!("Player {} to defend from {}", defender_index + 1, unresponded_attacks[unresponded_attack_index]);
                    match self.all_players[defender_index].get_move_as_defender(true, can_be_transfered) {
                        Action::Take => {
                            println!("Player {} takes", defender_index + 1);
                            self.all_players[defender_index].all_cards.append(&mut all_cards_thrown);
                            self.all_players[defender_index].all_cards.append(&mut unresponded_attacks);
                            self.player_index = Self::clamp(defender_index as i8 + 1, self.all_players.len()) as i8;
                            break;
                        }
                        Action::Defend(defence) => {
                            println!("Player {} defends with {}", defender_index + 1, defence);
                            all_cards_thrown.push(defence);
                            all_cards_thrown.push(unresponded_attacks.remove(unresponded_attack_index));
                            cards_thrown += 1;
                            can_be_transfered = false;
                        } 
                        Action::Transfer(transfer) => {
                            println!("Player {} transfers {} to player {}", defender_index + 1, transfer, defender_index + 2);
                            unresponded_attacks.push(transfer);
                            self.player_index = defender_index as i8;
                            cards_thrown += 1;
                        }
                        Action::Resign => {
                            println!("Player {} resigns", defender_index + 1);
                            panic!() // no resignation logic yet
                        }
                        _ => { //defender shouldn't attack or check
                            panic!();
                        }
                    }
                    unresponded_attack_index += 1;
                }

            }
            
            if is_beaten {
                self.is_first_beat = false;

                self.player_index = Self::clamp(self.player_index + 1, self.all_players.len()) as i8;
            }

            println!("Aftermath");           
            let mut can_defender_defend: bool = true;
            let mut index: i8 = 0;
            while index < self.all_players.len() as i8 { //after defender commited a move, other players can give him more cards
                index += 1;

                if self.is_cards_limit_exceeded(&cards_thrown) {
                    break;
                }

                let third_player_index = Self::clamp(self.player_index + index, self.all_players.len());
                match self.proccess_move_from_third(&mut unresponded_attacks, &third_player_index, &mut cards_thrown, &mut is_beaten) {
                    ThirdAttackResult::Attacked => {index -= 1; continue;} 
                    ThirdAttackResult::Beat => {} 
                }

                if can_defender_defend {
                    for unresponded_attack_index in 0..unresponded_attacks.len() {
                        println!("Player {} to defend from {}", self.player_index + 1, unresponded_attacks[unresponded_attack_index]);
                        match self.all_players[self.player_index as usize].get_move_as_defender(true, false) {
                            Action::Take => {
                                println!("Player {} takes", self.player_index + 1);
                                self.all_players[self.player_index as usize].all_cards.append(&mut all_cards_thrown);
                                can_defender_defend = false;
                            }
                            Action::Defend(defence) => {
                                println!("Player {} defends with {}", self.player_index + 1, defence);
                                all_cards_thrown.push(defence);
                                all_cards_thrown.push(unresponded_attacks.remove(unresponded_attack_index));
                                cards_thrown += 1;
                                index = 0;
                            } Action::Transfer(_) => {
                                panic!("Player somehow transfered");
                            }
                            Action::Resign => {
                                println!("Player {} resigns", self.player_index + 1);
                                panic!("Resignation") // no resignation logic yet
                            }
                            _ => { //defender shouldn't attack or check
                                panic!("Illegal actions");
                            }
                        }
                    }
                    
                }
            }
        }
        
    }

    fn proccess_move_from_third(&mut self, unresponded_attacks: &mut Vec<Card>, 
        third_player_index: &usize, cards_thrown: &mut u8, is_beaten: &mut bool) -> ThirdAttackResult {
        
        println!("Player {} to move", third_player_index + 1);
        match self.all_players[*third_player_index].get_move_as_third() {
            Action::Attack(attack) => {
                println!("Player {} attacks with {}", third_player_index + 1, attack);
                unresponded_attacks.push(attack);
                *cards_thrown += 1;
                ThirdAttackResult::Attacked
            }
            Action::Resign => {
                println!("Player {} resigns", third_player_index + 1);
                panic!(); //no resignation logic yet
            }
            Action::Check => {
                println!("Player {}: check", third_player_index + 1);
                *is_beaten = true;
                ThirdAttackResult::Beat 
            }
            _ => { // attacker shouldn't take, transfer or defend
                panic!();
            }
        }
    }

    fn is_cards_limit_exceeded(&self, cards_thrown: &u8) -> bool {
        !if self.is_first_beat { *cards_thrown / 2 < 4 } else { *cards_thrown / 2 < 5 }
    }

    fn clamp(num: i8, player_count: usize) -> usize { //prevents index out of bounds in game loop
        let result: usize = if num >= 0 {
            num as usize % player_count
        } else {
            (player_count - ((-num) as usize % player_count)) % player_count
        };
        
        result
    }

    pub fn new_card_stack(card_count: &CardCount) -> Vec<Card> {
        let mut card: u8;
        match card_count {
            CardCount::TwentyFour => {
                card = 8;
            }
            CardCount::ThirtySix => {
                card = 5;
            }
            CardCount::FiftyTwo => {
                card = 1;
            }
        }

        let mut cards: Vec<Card> = Vec::new();
        
        while card < 14 {
            let mut suit: u8 = 0;
            while suit < 4 {
                cards.push(Card::from_args(&suit, &card));
                suit += 1;
            }
            card += 1;
        }
        
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        cards
    }
}