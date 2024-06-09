#[derive(Debug)]
pub enum AftermathType {
    NoAftermath,
    RegularAftermath,
    TakingAftermath //happens when player takes a card: everyone else can give him more of the same cards
}

#[derive(Debug)]
pub enum RecentDefenderAction {
    NoAction,
    Defended,
    Took
}

#[derive(Debug)]
pub struct AftermathManager {
    aftermath_type: AftermathType,
    recent_action: RecentDefenderAction,
    start_index: usize,
    end_index: usize
}

impl AftermathManager {
    pub fn new() -> Self {
        AftermathManager{aftermath_type: AftermathType::NoAftermath, recent_action: RecentDefenderAction::NoAction, start_index: 0, end_index: 0}
    }

    pub fn set_type(&mut self, new_type: AftermathType) {
        self.aftermath_type = new_type;
    }

    pub fn set_recent_action(&mut self, recent_action: RecentDefenderAction) {
        self.recent_action = recent_action;
    }

    pub fn set_start_index(&mut self, new_index: usize) {
        self.start_index = new_index;
    }

    pub fn set_end_index(&mut self, new_index: usize) {
        self.end_index = new_index;
    }

    pub fn get_type(&mut self) -> &AftermathType {
        &self.aftermath_type
    }

    pub fn get_recent_action(&mut self) -> &RecentDefenderAction {
        &self.recent_action
    }

    pub fn get_start_index(&mut self) -> usize {
        self.start_index
    }

    pub fn get_end_index(&mut self) -> usize {
        self.end_index
    }

    pub fn reset(&mut self) {
        self.set_type(AftermathType::NoAftermath);
        self.set_recent_action(RecentDefenderAction::NoAction);
        self.set_start_index(0);
        self.set_end_index(0);
    }
}