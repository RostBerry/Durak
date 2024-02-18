use game_logic::card::CardCount;
use game_logic::game_manager::GameManager;

fn main() {
    let mut manager: GameManager = GameManager::new(6, CardCount::FiftyTwo);
    manager.run_game();
    
    
}
