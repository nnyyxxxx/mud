use crate::{constants::*, player::Player};

pub struct World {
    pub map: [[char; MAP_SIZE]; MAP_SIZE],
    pub players: Vec<Player>,
    pub current_player: usize,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            map: [['.'; MAP_SIZE]; MAP_SIZE],
            players: Vec::new(),
            current_player: 0,
        };

        world.players = (0..PLAYER_COUNT).map(Player::new).collect();
        world.place_gold();
        world
    }

    fn place_gold(&mut self) {
        let gold_positions = [(5, 5), (2, 7), (7, 3)];
        for (y, x) in gold_positions {
            self.map[y][x] = GOLD_TILE;
        }
    }

    pub fn is_adjacent(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let dx = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
        let dy = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
        dx + dy == 1
    }
}
