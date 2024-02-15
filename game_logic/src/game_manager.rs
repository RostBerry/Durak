use crate::{card::{Card, CardCount}, player::Player};
use rand::{seq::SliceRandom, Rng};
use rand::thread_rng;

pub struct GameManager {
    pub all_players: Vec<Player>,
    pub card_stack: Vec<Card>,
    pub card_count: CardCount,
    pub trump_suit: u8,
    pub is_first_beat: bool,
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
        GameManager{all_players, card_stack, card_count, trump_suit, is_first_beat: false}
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

        let mut index: i8 = 0;
        loop { //game loop
            let mut cards_thrown: u8 = 0;
            let mut all_cards_thrown: Vec<Card> = Vec::new();
            while !self.all_players[Self::clamp(index + 1, self.all_players.len())].all_cards.is_empty() &&
                    if self.is_first_beat { cards_thrown < 4 } else { cards_thrown < 5 } {
                
                let attacker_index = Self::clamp(index, self.all_players.len());
                let defender_index = Self::clamp(index + 1, self.all_players.len());

                println!("Player {} to move", attacker_index + 1);
                let attack: Card = self.all_players[attacker_index].get_move();
                println!("\n{}\n", attack);
                all_cards_thrown.push(attack);

                cards_thrown += 1;

                println!("Player {} to defend", defender_index + 1);
                let defence: Card = self.all_players[defender_index].get_move();
                println!("\n{}\n", defence);
                all_cards_thrown.push(defence);

            }


            index = Self::clamp(index + 1, self.all_players.len()) as i8;
        }
        
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
        
        while card < 13 {
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