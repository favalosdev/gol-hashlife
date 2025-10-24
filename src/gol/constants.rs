pub const WINDOW_HEIGHT: u32 = 600;
pub const WINDOW_WIDTH: u32 = 600;

pub const SQUARE_FACTOR: u8 = 5;
pub const SQUARE_SIZE: usize = (2 * SQUARE_FACTOR) as usize;
pub const N: usize = (WINDOW_HEIGHT / (SQUARE_SIZE as u32)) as usize;
pub const M: usize = (WINDOW_WIDTH / (SQUARE_SIZE as u32)) as usize;
pub const GAME_FREQ: u64 = 20;
