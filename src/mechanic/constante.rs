
pub const WIDTH: u32 = 1000;

pub const HEIGHT: u32 = 1000;

pub const SCALE: u32 = (WIDTH / 16)/10*10;

pub const RECT_CROSS: (i32, i32, u32, u32) = (
    (WIDTH / 2 - SCALE*3 + 1) as i32,
    (HEIGHT / 2 - SCALE*3 + 1) as i32,
    6 * SCALE - 2,
    6 * SCALE - 2,
);

pub const SPEED_RATE: i32 = 10;

pub const DRAW: (i32, i32, u32, u32) = (0, 0, WIDTH, HEIGHT);

pub const THROTTLE_DURATION: u32 = 1_000_000_000u32/4 ;


pub const FRAME: u32 = 1_000_000_000u32 / 60;
// pub const FRAME: u32 = 1_000_000_000u32 / 120;
// pub const FRAME: u32 = 1_000_000_000u32 / 240;