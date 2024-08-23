use std::collections::HashMap;
use std::time::Instant;

use crate::sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::video::*;

use super::constante::DRAW;
use super::constante::RECT_CROSS;
use super::constante::{HEIGHT, SCALE, WIDTH};
use super::impl_enum::CarColor;
use super::impl_enum::CarSpeed;
use super::impl_enum::CarTurn;
use super::impl_enum::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cars {
    pub instant: Instant,
    pub collider: Rect,
    pub direction: Direction,
    pub car_speed: CarSpeed,
    pub color: CarColor,
    pub direction_turn: CarTurn,
    pub turn: bool,
    pub check: bool,
    pub vmax: u32,
    pub vmin: u32,
}

impl Cars {
    // create a new cars based on the direction of x and y
    pub fn new(mut x: i32, mut y: i32, direction: Direction) -> Self {
        let mut car_speed = CarSpeed::Normal;
        let direction_turn = match rand::random::<CarTurn>() {
            CarTurn::Left => CarTurn::Left,
            CarTurn::Right => {
                match direction {
                    Direction::Down => {
                        x -= 2 * SCALE as i32;
                    }
                    Direction::Left => {
                        y -= 2 * SCALE as i32;
                    }
                    Direction::Right => {
                        y += 2 * SCALE as i32;
                    }
                    Direction::Up => {
                        x += 2 * SCALE as i32;
                    }
                }
                car_speed = CarSpeed::Rapide;
                CarTurn::Right
            }
            CarTurn::None => {
                match direction {
                    Direction::Down => {
                        x -= SCALE as i32;
                    }
                    Direction::Left => {
                        y -= SCALE as i32;
                    }
                    Direction::Right => {
                        y += SCALE as i32;
                    }
                    Direction::Up => {
                        x += SCALE as i32;
                    }
                }
                CarTurn::None
            }
        };
        Cars {
            instant: Instant::now(),
            collider: Rect::new(x, y, SCALE, SCALE),
            direction: direction,
            car_speed,
            color: rand::random::<CarColor>(),
            direction_turn: direction_turn,
            turn: false,
            check: false,
            vmax: 0,
            vmin: u32::MAX,
        }
    }

    //create a new car based on the option direction
    pub fn random(direction: Option<Direction>) -> Self {
        let direction = match direction {
            Some(d) => d,
            None => rand::random::<Direction>(),
        };
        let (x, y) = match direction {
            Direction::Down => ((WIDTH / 2 - SCALE) as i32, 0 - SCALE as i32),
            Direction::Up => ((WIDTH / 2) as i32, (HEIGHT) as i32),
            Direction::Left => ((WIDTH - SCALE) as i32, (HEIGHT / 2 - SCALE) as i32),
            Direction::Right => (0 - (SCALE) as i32, (HEIGHT / 2) as i32),
        };
        Cars::new(x, y, direction)
    }

    // check whether a car can be placed on the intersection
    pub fn insertion(&mut self, tab_cars: &Vec<Cars>) -> bool {
        let mut move_rect = self.collider.clone();
        match self.direction {
            Direction::Down | Direction::Up => {
                move_rect.set_width(SCALE);
                move_rect.set_height(SCALE * 2);
                if self.direction == Direction::Up {
                    move_rect = move_rect.top_shifted(SCALE as i32 * 2);
                } else {
                    move_rect = move_rect.bottom_shifted(SCALE as i32);
                }
            }
            Direction::Right | Direction::Left => {
                move_rect.set_width(SCALE * 2);
                move_rect.set_height(SCALE);
                if self.direction == Direction::Left {
                    move_rect = move_rect.left_shifted(SCALE as i32 * 2);
                } else {
                    move_rect = move_rect.right_shifted(SCALE as i32);
                }
            }
        }
        for index in 0..tab_cars.len() {
            let mut car = tab_cars[index].clone();
            match car.direction {
                Direction::Down => {
                    car.collider = car.collider.bottom_shifted(self.car_speed.get_speed());
                }
                Direction::Up => {
                    car.collider = car.collider.top_shifted(self.car_speed.get_speed());
                }
                Direction::Right => {
                    car.collider = car.collider.right_shifted(self.car_speed.get_speed());
                }
                Direction::Left => {
                    car.collider = car.collider.left_shifted(self.car_speed.get_speed());
                }
            };
            if move_rect.has_intersection(car.collider) {
                return false;
            }
        }
        return true;
    }

    // check that no car is blocking the next movement of the test car according to the table entered as a parameter
    pub fn check_colision(&mut self, tab_cars: &Vec<Cars>, self_index: usize) -> bool {
        if !self.turn {
            self.rotate();
        }
        if self.direction_turn == CarTurn::Right && self_index != usize::MAX {
            return true;
        }
        let mut temp = self.clone();
        temp.forward();
        let mut move_rect = temp.collider.clone();
        let mut stop_rect = self.collider.clone();

        match self.direction {
            Direction::Down | Direction::Up => {
                move_rect.set_width(SCALE);
                move_rect.set_height(SCALE * 2);
                stop_rect.set_height(SCALE / 10);
                stop_rect.set_width(SCALE - 12);
                stop_rect.x += 6;

                if self.direction == Direction::Up {
                    move_rect = move_rect.top_shifted(SCALE as i32 * 2);
                    stop_rect.y -= 2 + self.car_speed.get_speed();
                } else {
                    move_rect = move_rect.bottom_shifted(SCALE as i32);
                    stop_rect.y += 2 + SCALE as i32 + self.car_speed.get_speed();
                }
            }
            Direction::Right | Direction::Left => {
                move_rect.set_width(SCALE * 2);
                stop_rect.set_width(SCALE / 10);
                stop_rect.set_height(SCALE - 12);
                move_rect.set_height(SCALE);
                stop_rect.y += 6;
                if self.direction == Direction::Left {
                    move_rect = move_rect.left_shifted(SCALE as i32 * 2);

                    stop_rect.x = temp.collider.x - 2;
                } else {
                    move_rect = move_rect.right_shifted(SCALE as i32);

                    stop_rect.x = temp.collider.x + SCALE as i32 + 2;
                }
            }
        }

        let mut check = 0;
        let mut left = 0;
        let canva_rect = Rect::from(RECT_CROSS);
        for index in 0..tab_cars.len() {
            if self_index == index {
                continue;
            }
            let car = tab_cars[index].clone();
            if (car.check || canva_rect.contains_rect(car.collider))
                && car.direction_turn == CarTurn::Left
            {
                left += 1;
            }
            if move_rect.has_intersection(car.collider) && check < 1 {
                check = 1;
            }
            if (stop_rect.has_intersection(car.collider) || car.collider.contains_rect(stop_rect))
                && check < 2
            {
                check = 2;
            }
        }
        match check {
            0 => {
                if self.turn {
                    self.car_speed = CarSpeed::Rapide;
                } else {
                    self.car_speed = CarSpeed::Normal;
                }
            }
            1 => {
                self.car_speed = CarSpeed::Lent;
            }
            2 => {
                self.car_speed = CarSpeed::Stop;
            }
            _ => {}
        }
        if self.direction_turn == CarTurn::Left
            && left >= 3
            && canva_rect.has_intersection(self.collider)
            && !self.check
        {
            check = 2;
        } else if canva_rect.has_intersection(self.collider) {
            self.check = true;
        }
        return check < 2;
    }

    // function called at each iteration of the screen loop
    pub fn tick(
        &mut self,
        canvas: &mut Canvas<Window>,
        tab_cars: &mut Vec<Cars>,
        index: usize,
        tab_texture: &HashMap<CarColor, Texture>,
    ) -> bool {
        if self.check_colision(tab_cars, index) {
            if true {
                self.forward();
                if !self.turn {
                    self.rotate();
                }
            }
        }
        let speed = self.car_speed.get_speed() as u32;
        if self.vmax < speed {
            self.vmax = speed;
        }
        if self.vmin > speed {
            self.vmin = speed;
        }
        let canva_rect = Rect::new(
            DRAW.0 - (SCALE) as i32,
            DRAW.1 - (SCALE) as i32,
            DRAW.2 + SCALE,
            DRAW.3 + SCALE,
        );
        tab_cars[index] = self.clone();
        if canva_rect.intersection(self.collider).is_none() {
            false
        } else {
            self.draw(canvas, tab_texture);
            true
        }
    }

    // function to move the car in the direction it needs to go
    pub fn forward(&mut self) {
        match self.direction {
            Direction::Down => {
                self.collider = self.collider.bottom_shifted(self.car_speed.get_speed());
            }
            Direction::Up => {
                self.collider = self.collider.top_shifted(self.car_speed.get_speed());
            }
            Direction::Right => {
                self.collider = self.collider.right_shifted(self.car_speed.get_speed());
            }
            Direction::Left => {
                self.collider = self.collider.left_shifted(self.car_speed.get_speed());
            }
        };
    }

    // function used to rotate the car
    pub fn rotate(&mut self) {
        match self.direction_turn {
            CarTurn::Left => match self.direction {
                Direction::Down => {
                    if self.collider.y >= (HEIGHT / 2) as i32 {
                        self.collider.y = (HEIGHT / 2) as i32;
                        self.direction = Direction::Right;
                        self.turn = true;
                    }
                }
                Direction::Up => {
                    if self.collider.y <= (HEIGHT / 2 - SCALE) as i32 {
                        self.collider.y = (HEIGHT / 2 - SCALE) as i32;
                        self.direction = Direction::Left;
                        self.turn = true;
                    }
                }
                Direction::Left => {
                    if self.collider.x < (WIDTH / 2 - SCALE) as i32 {
                        self.collider.x = (WIDTH / 2 - SCALE) as i32;
                        self.direction = Direction::Down;
                        self.turn = true;
                    }
                }
                Direction::Right => {
                    if self.collider.x >= (WIDTH / 2) as i32 {
                        self.collider.x = (WIDTH / 2) as i32;
                        self.direction = Direction::Up;
                        self.turn = true;
                    }
                }
            },
            CarTurn::Right => match self.direction {
                Direction::Down => {
                    if self.collider.y >= (HEIGHT / 2 - 3 * SCALE) as i32 {
                        self.collider.y = (HEIGHT / 2 - 3 * SCALE) as i32;
                        self.direction = Direction::Left;
                        self.turn = true;
                    }
                }
                Direction::Up => {
                    if self.collider.y <= (HEIGHT / 2 + 2 * SCALE) as i32 {
                        self.collider.y = (HEIGHT / 2 + 2 * SCALE) as i32;
                        self.direction = Direction::Right;
                        self.turn = true;
                    }
                }
                Direction::Left => {
                    if self.collider.x <= (WIDTH / 2 + 2 * SCALE) as i32 {
                        self.collider.x = (WIDTH / 2 + 2 * SCALE) as i32;
                        self.direction = Direction::Up;
                        self.turn = true;
                    }
                }
                Direction::Right => {
                    if self.collider.x >= (WIDTH / 2 - 3 * SCALE) as i32 {
                        self.collider.x = (WIDTH / 2 - 3 * SCALE) as i32;
                        self.direction = Direction::Down;
                        self.turn = true;
                    }
                }
            },
            CarTurn::None => match self.direction {
                Direction::Down => {
                    if self.collider.y == (HEIGHT / 2) as i32 {
                        self.turn = true;
                    }
                }
                Direction::Up => {
                    if self.collider.y == (HEIGHT / 2 - SCALE) as i32 {
                        self.turn = true;
                    }
                }
                Direction::Left => {
                    if self.collider.x == (WIDTH / 2 - SCALE) as i32 {
                        self.turn = true;
                    }
                }
                Direction::Right => {
                    if self.collider.x == (WIDTH / 2) as i32 {
                        self.turn = true;
                    }
                }
            },
        }
    }

    // function that prints/displays the car on the screen
    pub fn draw(&self, canvas: &mut Canvas<Window>, tab_texture: &HashMap<CarColor, Texture>) {
        let texture = tab_texture.get(&self.color).unwrap();
        let angle = match self.direction {
            Direction::Down => -90.0,
            Direction::Up => 90.0,
            Direction::Left => 0.0,
            Direction::Right => 180.0,
        };
        let mut draw = self.collider.clone();
        draw.offset(0, (SCALE / 4) as i32);
        draw.resize(SCALE, SCALE / 2);
        canvas
            .copy_ex(texture, None, Some(draw), angle, None, false, false)
            .unwrap();
    }
}

// function that returns a hashmap storing the different textures of the cars
pub fn init_texture(texture_creator: &TextureCreator<WindowContext>) -> HashMap<CarColor, Texture> {
    let mut hash_map: HashMap<CarColor, Texture> = HashMap::new();
    hash_map.insert(
        CarColor::Red,
        texture_creator
            .load_texture(format!("./assets/red_car.png"))
            .unwrap(),
    );
    hash_map.insert(
        CarColor::Yellow,
        texture_creator
            .load_texture(format!("./assets/yellow_car.png"))
            .unwrap(),
    );
    hash_map.insert(
        CarColor::Green,
        texture_creator
            .load_texture(format!("./assets/green_car.png"))
            .unwrap(),
    );
    hash_map
}
