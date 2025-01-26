pub const MAP_SIZE: usize = 10;
pub const PLAYER_COUNT: usize = 2;
pub const ATTACK_DAMAGE: i32 = 25;
pub const GOLD_VALUE: i32 = 10;
pub const TOTAL_GOLD_PIECES: i32 = 3;
pub const GOLD_TO_WIN: i32 = TOTAL_GOLD_PIECES * GOLD_VALUE;

pub const EMPTY_TILE: char = '.';
pub const GOLD_TILE: char = 'G';
pub const PLAYER_CHARS: [char; PLAYER_COUNT] = ['1', '2'];

pub const MSG_INPUT_PROMPT: &str = "Move (wasd), attack (f), or quit (q): ";

pub const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1H";
