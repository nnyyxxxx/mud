use crate::constants::*;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub health: i32,
    pub gold: i32,
}

impl Player {
    pub fn new(index: usize) -> Self {
        Self {
            x: index * (MAP_SIZE - 1),
            y: 0,
            health: 100,
            gold: 0,
        }
    }

    pub fn handle_movement(&mut self, direction: char) -> bool {
        match direction {
            'w' if self.y > 0 => self.y -= 1,
            's' if self.y < MAP_SIZE - 1 => self.y += 1,
            'a' if self.x > 0 => self.x -= 1,
            'd' if self.x < MAP_SIZE - 1 => self.x += 1,
            _ => return false,
        }
        true
    }
}
