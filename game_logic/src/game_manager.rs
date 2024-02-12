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

        while iteration_count < 6 {
            for player in self.all_players.iter_mut() {
                if let Some(card) = self.card_stack.pop() {
                    player.give_card(card);
                }
            }
            iteration_count += 1;
        }
        
        match self.card_stack.first() {
            Some(card) => {
                println!("Last card: {}", card.to_string());
                self.trump_suit = card.suit();
            },
            None => panic!()
        }

        let mut index: i8 = 0;
        loop {
            let mut cards_thrown: u8 = 0;
            while !self.all_players[Self::clamp(index + 1)].all_cards.is_empty() || 
                    if self.is_first_beat { cards_thrown < 5 } else { cards_thrown < 6 } {
                let attack: &Card = self.all_players[Self::clamp(index)].get_move();
                println!("{}", attack.to_string());
                let defence: &Card = self.all_players[Self::clamp(index + 1)].get_move();
                println!("{}", defence.to_string());
                cards_thrown += 1;
            }


            index += 1;
            if index >= 6 {
                index = 0;
            }
        }
        
    }

    fn clamp(num: i8) -> usize {
        let result: usize = if num >= 0 {
            num as usize % 6
        } else {
            (6 - ((-num) as usize % 6)) % 6
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