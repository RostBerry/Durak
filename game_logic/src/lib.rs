pub mod card;
pub mod player;
pub mod game_manager;

#[cfg(test)]
mod tests {
    use crate::game_manager::GameManager;
    use crate::card::CardCount;


    #[test]
    fn game_start() {
        let manager: GameManager = GameManager::new(6, CardCount::FiftyTwo);
        println!("{:?}", manager.all_players);
        for player in manager.all_players {
            println!("{:?}", player.all_cards);
        }
    }
}
