use crate::{card::{Card, CardCount}, player::Player};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct GameManager {
    pub all_players: Vec<Player>,
    pub card_stack: Vec<Card>,
    pub card_count: CardCount
}

impl GameManager {

    pub fn new(mut player_count: u8, card_count: CardCount) -> GameManager {
        let mut all_players: Vec<Player> = Vec::new();
        while player_count > 0 {
            all_players.push(Player::new());
            player_count -= 1;
        }
        GameManager{all_players, card_stack: Self::new_card_stack(&card_count), card_count}
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
        
    }

    pub fn new_card_stack(card_count: &CardCount) -> Vec<Card> {
        let mut card: u8;
        match card_count {
            CardCount::TwentyFour => {
                card = 9;
            }
            CardCount::ThirtySix => {
                card = 6;
            }
            CardCount::FiftyTwo => {
                card = 2;
            }
        }

        let mut cards: Vec<Card> = Vec::new();
        
        while card < 14 {
            let mut suit: u8 = 0;
            while suit < 4 {
                cards.push(Card::from_args(&suit, &((suit == 3) as u8), &card));
                suit += 1;
            }
            card += 1;
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        cards
    }
}