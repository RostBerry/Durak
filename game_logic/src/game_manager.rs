use crate::{aftermath_manager::{AftermathType, AftermathManager, RecentDefenderAction}, card::{Card, CardCount}, card_pair::CardPair, player::Player};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub enum Action {
    Attack(CardPair),
    Defend(Card),
    Check,
    Transfer(CardPair),
    Resign,
    Take
}

pub enum PlayerType {
    Attacker,
    Defender,
    AdditionalAttacker, //appears when attacker attacked: he can throw more cards
    AftermathAttacker //appears when aftermath
}

pub struct GameManager {
    pub all_players: Vec<Player>,
    player_count: usize,
    pub card_stack: Vec<Card>,
    pub card_count: CardCount,
    pub trump_suit: u8,
    pub is_first_beat: bool,
    player_index: usize 

}

impl GameManager {

    pub fn new(player_count: usize, card_count: CardCount) -> GameManager {
        let mut all_players: Vec<Player> = Vec::new(); 

        let mut players: usize = player_count;
        while players > 0 { //creating players
            all_players.push(Player::new()); 
            players -= 1;
        }

        let card_stack: Vec<Card> = Self::new_card_stack(&card_count); //creating cards deck
        let trump_suit;
        match card_stack.first() { //defining the trump
            Some(card) => {
                println!("\nLast card: {}", card);
                for i in 0..11 { //printing the last card
                    println!("{}", card[i]);
                }
                trump_suit = card.suit();
            },
            None => panic!() 
        }
        GameManager{all_players, player_count, card_stack, card_count, trump_suit, is_first_beat: false, player_index: 0}
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

        let mut cards_thrown: u8 = 0; //counter to disallow throwing cards if there is already 5 or 6 on the board, or if the defender has less
        let mut all_card_pairs: Vec<CardPair> = Vec::new(); //temp place for all attacks
        let mut player_type: PlayerType = PlayerType::Attacker; //decides if the next player will be an attacker, a defender or a third player
        let mut aftermath_manager: AftermathManager = AftermathManager::new(); // manages the aftermath and it's type

        loop { //game loop

            match player_type {

                PlayerType::Attacker => {

                    if self.all_players[self.player_index].does_have_cards() && //attacker has cards
                    !self.is_cards_limit_exceeded(&cards_thrown) { //attackers did not throw too much cards

                        println!("Player {} to attack", self.player_index + 1);

                        match self.all_players[self.player_index].get_move_from_attacker() {

                            Action::Attack(attack) => {

                                println!("Player {} attacks with {}", self.player_index + 1, attack);
                                cards_thrown += 1;
                                all_card_pairs.push(attack);

                                player_type = PlayerType::AdditionalAttacker; //attacker can throw more than 1 card at a time, thus he can do it in the next iteration
                                //not switching players, current players becomes a third player
        
                            }

                            Action::Resign => {
                                
                                println!("Player {} resigns", self.player_index + 1);
                                panic!(); //no resignation logic yet
                            }
                            _ => { // attacker shouldn't take, transfer, check or defend in the first attack
                                panic!();
                            }
                        }
                    }
                },

                PlayerType::Defender => {

                    if self.all_players[self.player_index].does_have_cards() { //defender has cards

                        let mut is_taken: bool = false;
                        let mut can_be_transfered: bool = true;
                        let mut transfer_pair: Option<CardPair> = None; // if the player transfers, the transfered pair will then be pushed to the attacks
    
                        for attack in all_card_pairs.iter_mut() {
                            if attack.is_filled() { // already responded attack should not be addressed
                                continue;
                            }
                            println!("Player {} to defend from {}", self.player_index + 1, attack);
    
                            match self.all_players[self.player_index].get_move_from_defender(true, can_be_transfered) {
                                
                                Action::Take => {
                                    println!("Player {} takes", self.player_index + 1);
                                    is_taken = true; //cycle obrupts and all card pairs are given to the defender
                                    break;
                                },
    
                                Action::Defend(defense) => {
                                    println!("Player {} defends with {}", self.player_index + 1, defense);
                                    attack.fill(defense);
                                    can_be_transfered = false;
                                    aftermath_manager.set_recent_action(RecentDefenderAction::Defended);
                                },
    
                                Action::Transfer(transfer) => {
                                    println!("Player {} transfers with {}", self. player_index + 1, transfer);
                                    transfer_pair = Some(transfer);
                                    cards_thrown += 1;
    
                                    break; //cycle obrupts and transfered card pair is going to the next player
                                },
    
                                Action::Resign => {
                                    panic!("resignation") //no resignation logic yet
                                },
    
                                _ => {
                                    panic!("Impossible option");
                                }
    
                            }
                        }
    
                        if is_taken { // player takes all cards
                            let pairs_to_give = all_card_pairs.drain(..).collect::<Vec<_>>();
                            self.all_players[self.player_index].give_card_pairs(pairs_to_give);  
                            self.increment_player(true);
                            player_type = PlayerType::Attacker; // switching the player, the next player is attacker
                            aftermath_manager.set_recent_action(RecentDefenderAction::Took);
                            continue;
                        }
    
                        if let Some(transfer) = transfer_pair { //player transfered
                            all_card_pairs.push(transfer);
                            player_type = PlayerType::AdditionalAttacker; // defender becomes a third player
                            continue;
                        } 
                        
                        self.increment_player(false);
                        player_type = PlayerType::AdditionalAttacker; //switching players, the next player is a third player  
                        
                    }
                },

                PlayerType::AdditionalAttacker => {
                    println!("{:?}", aftermath_manager);


                    if !self.is_cards_limit_exceeded(&cards_thrown) && //attackers did not throw too mush cards
                    self.all_players[self.player_index].does_have_cards() { //attacker has cards

                        println!("Player {} to make an additional attack", self.player_index + 1);

                        match self.all_players[self.player_index].get_move_from_third() {
                            Action::Attack(attack) => {
                                println!("Player {} attacks with {}", self.player_index + 1, attack);
                                all_card_pairs.push(attack);
                                cards_thrown += 1;
                                aftermath_manager.set_recent_action(RecentDefenderAction::NoAction);
                                // not switching players, the player will be still a third player in the next loop iteration
                            },

                            Action::Check => {

                                if self.get_active_player_count() > 2 {

                                    match aftermath_manager.get_type() {

                                        AftermathType::NoAftermath => {
    
                                            match aftermath_manager.get_recent_action() {
    
                                                RecentDefenderAction::NoAction => {
    
                                                    self.increment_player(true);
                                                    player_type = PlayerType::Defender; //switching players, the next player will be a defender
    
                                                },
    
                                                RecentDefenderAction::Defended => {
                                                    
                                                    aftermath_manager.set_end_index(self.get_nearest_player(false));
                                                    for _ in 0..2 {
                                                        self.increment_player(true);
                                                    }
                                                    player_type = PlayerType::Third; //switching players, the next player will be a third player
                                                    aftermath_manager.set_type(AftermathType::RegularAftermath);
                                                    aftermath_manager.set_start_index(self.player_index);
                                                },
    
                                                RecentDefenderAction::Took => {
    
                                                    aftermath_manager.set_end_index(self.get_nearest_player(false));
                                                    for _ in 0..2 {
                                                        self.increment_player(true);
                                                    }
                                                    player_type = PlayerType::Third; //switching players, the next player is a third player
                                                    aftermath_manager.set_type(AftermathType::TakingAftermath);
                                                    aftermath_manager.set_start_index(self.player_index);
                                                }
                                            }
                                        },
    
                                        AftermathType::RegularAftermath => {
                                            println!("Aftermath");
    
                                            if self.player_index == aftermath_manager.get_end_index() {
    
                                                self.increment_player(true);
                                                player_type = PlayerType::Defender; //the next player is a defender
                                                aftermath_manager.reset();
    
                                            } else {
    
                                                self.increment_player(true);
                                                player_type = PlayerType::Third; //the next player is a third player

                                            }
                                        },
    
                                        AftermathType::TakingAftermath => {
    
                                            if self.player_index == aftermath_manager.get_end_index() {
    
                                                self.increment_player(true);
                                                let pairs_to_give = all_card_pairs.drain(..).collect::<Vec<_>>();
                                                self.all_players[self.player_index].give_card_pairs(pairs_to_give);  
                                                self.increment_player(true);
                                                player_type = PlayerType::Attacker; // switching the player, the next player is attacker

                                            } else {
    
                                                self.increment_player(true);
                                                player_type = PlayerType::Third; //the next player is a third player
                                            }
                                        }
                                    }
                                } else {

                                    match aftermath_manager.get_recent_action() {

                                        RecentDefenderAction::Defended => {
                                            self.increment_player(true);
                                            player_type = PlayerType::Defender; //the next player is a defender
                                            self.send_attacks_to_drop_stack(&mut all_card_pairs)
                                        },

                                        RecentDefenderAction::Took => {

                                            let pairs_to_give = all_card_pairs.drain(..).collect::<Vec<_>>();
                                            self.all_players[self.player_index].give_card_pairs(pairs_to_give); 
                                            // since there are only two players and one took, it's still the current player's turn
                                        },

                                        _ => {
                                            panic!("Impossible action");
                                        }
                                    }
                                }

                            }, 

                            Action::Resign => {
                                panic!("resignation"); //no resignation logic yet
                            },

                            _ => {
                                panic!("impossible option");
                            }
                        }

                    }
                },

                // PlayerType::AdditionalAttacker => {
                //     println!("{:?}", aftermath_manager);


                //     if !self.is_cards_limit_exceeded(&cards_thrown) && //attackers did not throw too mush cards
                //     self.all_players[self.player_index].does_have_cards() { //attacker has cards

                //         println!("Player {} to make an additional attack", self.player_index + 1);

                //         match self.all_players[self.player_index].get_move_from_third() {
                //             Action::Attack(attack) => {
                //                 println!("Player {} attacks with {}", self.player_index + 1, attack);
                //                 all_card_pairs.push(attack);
                //                 cards_thrown += 1;
                //                 aftermath_manager.set_recent_action(RecentDefenderAction::NoAction);
                //                 // not switching players, the player will be still a third player in the next loop iteration
                //             },

                //             Action::Check => {

                //                 if self.get_active_player_count() > 2 {

                //                     match aftermath_manager.get_type() {

                //                         AftermathType::NoAftermath => {
    
                //                             match aftermath_manager.get_recent_action() {
    
                //                                 RecentDefenderAction::NoAction => {
    
                //                                     self.increment_player(true);
                //                                     player_type = PlayerType::Defender; //switching players, the next player will be a defender
    
                //                                 },
    
                //                                 RecentDefenderAction::Defended => {
                                                    
                //                                     aftermath_manager.set_end_index(self.get_nearest_player(false));
                //                                     for _ in 0..2 {
                //                                         self.increment_player(true);
                //                                     }
                //                                     player_type = PlayerType::Third; //switching players, the next player will be a third player
                //                                     aftermath_manager.set_type(AftermathType::RegularAftermath);
                //                                     aftermath_manager.set_start_index(self.player_index);
                //                                 },
    
                //                                 RecentDefenderAction::Took => {
    
                //                                     aftermath_manager.set_end_index(self.get_nearest_player(false));
                //                                     for _ in 0..2 {
                //                                         self.increment_player(true);
                //                                     }
                //                                     player_type = PlayerType::Third; //switching players, the next player is a third player
                //                                     aftermath_manager.set_type(AftermathType::TakingAftermath);
                //                                     aftermath_manager.set_start_index(self.player_index);
                //                                 }
                //                             }
                //                         },
    
                //                         AftermathType::RegularAftermath => {
                //                             println!("Aftermath");
    
                //                             if self.player_index == aftermath_manager.get_end_index() {
    
                //                                 self.increment_player(true);
                //                                 player_type = PlayerType::Defender; //the next player is a defender
                //                                 aftermath_manager.reset();
    
                //                             } else {
    
                //                                 self.increment_player(true);
                //                                 player_type = PlayerType::Third; //the next player is a third player

                //                             }
                //                         },
    
                //                         AftermathType::TakingAftermath => {
    
                //                             if self.player_index == aftermath_manager.get_end_index() {
    
                //                                 self.increment_player(true);
                //                                 let pairs_to_give = all_card_pairs.drain(..).collect::<Vec<_>>();
                //                                 self.all_players[self.player_index].give_card_pairs(pairs_to_give);  
                //                                 self.increment_player(true);
                //                                 player_type = PlayerType::Attacker; // switching the player, the next player is attacker

                //                             } else {
    
                //                                 self.increment_player(true);
                //                                 player_type = PlayerType::Third; //the next player is a third player
                //                             }
                //                         }
                //                     }
                //                 } else {

                //                     match aftermath_manager.get_recent_action() {

                //                         RecentDefenderAction::Defended => {
                //                             self.increment_player(true);
                //                             player_type = PlayerType::Defender; //the next player is a defender
                //                             self.send_attacks_to_drop_stack(&mut all_card_pairs)
                //                         },

                //                         RecentDefenderAction::Took => {

                //                             let pairs_to_give = all_card_pairs.drain(..).collect::<Vec<_>>();
                //                             self.all_players[self.player_index].give_card_pairs(pairs_to_give); 
                //                             // since there are only two players and one took, it's still the current player's turn
                //                         },

                //                         _ => {
                //                             panic!("Impossible action");
                //                         }
                //                     }
                //                 }

                //             }, 

                //             Action::Resign => {
                //                 panic!("resignation"); //no resignation logic yet
                //             },

                //             _ => {
                //                 panic!("impossible option");
                //             }
                //         }

                //     }
                // }
            }
        }

        
        
    }

    fn is_cards_limit_exceeded(&self, cards_thrown: &u8) -> bool { 
        !if self.is_first_beat { *cards_thrown / 2 < 4 } else { *cards_thrown / 2 < 5 }
    }

    fn increment_player(&mut self, direction: bool) { //prevents index out of bounds in game loop
        self.player_index = self.get_nearest_player(direction);
    }

    fn get_incremented_index_by_value(&mut self, index: usize, value: usize) -> usize {
        (index + value) % self.player_count
    }

    fn send_attacks_to_drop_stack(&mut self, attacks: &mut Vec<CardPair>) {
        attacks.clear();
    }

    fn get_nearest_player(&mut self, direction: bool) -> usize { // finds nearest player still in the game to the current player

        if !self.all_players[self.player_index].does_have_cards() {
            panic!("index error, current player doesn't have cards");
        }
        let mut output_index: usize = self.player_index;
        loop {

            if direction {
                output_index = self.get_incremented_index_by_value(output_index, 1);
            } else {
                output_index = self.get_incremented_index_by_value(output_index, self.player_count - 1);
            }

            if self.all_players[output_index].does_have_cards() {

                if output_index == self.player_index {
                    panic!("there are no more players left");
                }
                return output_index;
            }
        }
    }

    fn get_active_player_count(&mut self) -> usize {
        let mut output = 0;
        for player in &self.all_players {
            if player.does_have_cards() {
                output += 1;
            }
        }
        output
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
                cards.push(Card::from_args(suit, card));
                suit += 1;
            }
            card += 1;
        }
        
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        cards
    }
}