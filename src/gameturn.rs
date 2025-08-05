

pub struct GameTurn {
    pub turn: u32,
    pub max_turn: u32,
}

impl GameTurn {
    pub fn new(max_turn: u32) -> Self {
        Self { turn: 0, max_turn }
    }
    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    pub fn reset_turn(&mut self) {
        self.turn = 0;
    }

    pub fn turn_over(&self) -> bool {
        self.turn >= self.max_turn
    }

    pub fn player_turn(&self) -> bool {
        self.turn % 2 == 0
    }

    pub fn enemy_turn(&self) -> bool {
        self.turn % 2 == 1
    }
}


