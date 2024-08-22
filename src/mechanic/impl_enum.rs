use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::constante::SPEED_RATE;

/////////////////
/// Direction ///
/////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!(""),
        }
    }
}

/////////////////
/// Car Color ///
/////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CarColor {
    Red,
    Yellow,
    Green,
}

impl Distribution<CarColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CarColor {
        match rng.gen_range(0..=2) {
            0 => CarColor::Red,
            1 => CarColor::Yellow,
            2 => CarColor::Green,
            _ => panic!(""),
        }
    }
}

////////////////
/// Car Turn ///
////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CarTurn {
    Right,
    Left,
    None,
}

impl Distribution<CarTurn> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CarTurn {
        match rng.gen_range(0..=2) {
            0 => CarTurn::Right,
            1 => CarTurn::Left,
            2 => CarTurn::None,
            _ => panic!(""),
        }
    }
}

/////////////////
/// Car Speed ///
/////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CarSpeed {
    Stop,
    Treslent,
    Lent,
    Normal,
    Rapide,
    Tresrapide,
}

// impl Distribution<CarSpeed> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CarSpeed {
//         match rng.gen_range(0..=3) {
//             0 => CarSpeed::Stop,
//             1 => CarSpeed::Lent,
//             2 => CarSpeed::Normal,
//             3 => CarSpeed::Rapide,
//             _ => panic!(""),
//         }
//     }
// }

impl CarSpeed {
    pub fn get_Speed(self) -> i32 {
        match self {
            CarSpeed::Stop => 0,
            CarSpeed::Treslent => SPEED_RATE / 4,
            CarSpeed::Lent => SPEED_RATE / 2,
            CarSpeed::Normal => SPEED_RATE,
            CarSpeed::Rapide => SPEED_RATE * 2,
            CarSpeed::Tresrapide => SPEED_RATE * 4,
        }
    }
}
