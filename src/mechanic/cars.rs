use std::collections::HashMap;

use crate::sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::video::*;

use super::constante::DRAW;
use super::constante::SPEED_RATE;
use super::constante::{HEIGHT, SCALE, WIDTH};
use super::impl_enum::CarColor;
use super::impl_enum::CarTurn;
use super::impl_enum::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cars {
    pub collider: Rect,
    pub direction: Direction,
    pub speed_rate: i32,
    pub color: CarColor,
    pub direction_turn: CarTurn,
    pub turn: bool,
}

impl Cars {
    pub fn new(mut x: i32, mut y: i32, direction: Direction) -> Self {
        let mut speed_rate = SPEED_RATE;
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
                speed_rate *= 2;
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
            collider: Rect::new(x, y, SCALE, SCALE),
            direction: direction,
            speed_rate,
            color: rand::random::<CarColor>(),
            direction_turn: direction_turn,
            turn: false,
        }
    }

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

    pub fn check_colision(
        &mut self,
        tab_cars: &Vec<Cars>,
        self_index: usize,
        canvas: &mut Canvas<Window>,
        display: bool,
    ) -> bool {
        if self.direction_turn == CarTurn::Right {
            return true;
        }
        // let (x, y) = match self.direction {
        //     Direction::Down => (self.collider.x + 1, self.collider.y + SPEED_RATE * 2),
        //     Direction::Up => (self.collider.x + 1, self.collider.y - SPEED_RATE * 2),
        //     Direction::Left => (self.collider.x - SPEED_RATE * 2, self.collider.y + 1),
        //     Direction::Right => (self.collider.x + SPEED_RATE * 2, self.collider.y + 1),
        // };
        // let self_rect = Rect::new(x, y, SCALE - 2, SCALE - 2);
        let mut self_rect = self.collider.clone();
        self_rect.x += (SCALE / 8) as i32;
        self_rect.y += (SCALE / 8) as i32;

        // let mut stop_rect = self_rect.clone();
        let mut stop_rect = self.collider.clone();
        // 80%
        match self.direction {
            Direction::Down | Direction::Up => {
                self_rect.set_width(SCALE - SCALE / 8 * 2);
                self_rect.set_height(SCALE * 4 - SCALE / 8 * 2);
                if self.direction == Direction::Up {
                    self_rect.y -= 3 * (SCALE) as i32 + 10;
                } else {
                    self_rect.y += 10;
                }
            }
            Direction::Right | Direction::Left => {
                self_rect.set_width(SCALE * 4 - SCALE / 8 * 2);
                self_rect.set_height(SCALE - SCALE / 8 * 2);
                if self.direction == Direction::Left {
                    self_rect.x -= 3 * (SCALE) as i32 + 10;
                } else {
                    self_rect.x += 10;
                }
            }
        }

        // arret
        // match self.direction {
        //     Direction::Down | Direction::Up => {
        //         stop_rect.set_width(SCALE - SCALE / 8 * 2);
        //         stop_rect.set_height(SCALE * 2 - SCALE / 8 * 2);
        //         if self.direction == Direction::Up {
        //             stop_rect.y -= 1 * (SCALE) as i32;
        //         }
        //     }
        //     Direction::Right | Direction::Left => {
        //         stop_rect.set_width(SCALE * 2 - SCALE / 8 * 2);
        //         stop_rect.set_height(SCALE - SCALE / 8 * 2);
        //         if self.direction == Direction::Left {
        //             stop_rect.x -= 1 * (SCALE) as i32;
        //         }
        //     }
        // }
        stop_rect.set_width(SCALE + 2);
        stop_rect.set_height(SCALE + 2);
        match self.direction {
            Direction::Down => stop_rect.y += 1,// * (SCALE) as i32,
            Direction::Up => stop_rect.y -= 1,// * (SCALE) as i32,
            Direction::Right => stop_rect.x += 1,// * (SCALE) as i32,
            Direction::Left => stop_rect.x -= 1,// * (SCALE) as i32,
        }

        // self_rect.set_width(self_rect.width() - SCALE / 8 * 2);
        // self_rect.set_height(self_rect.height() - SCALE / 8 * 2);

        match self.direction {
            Direction::Down => {
                self_rect = self_rect.bottom_shifted(SPEED_RATE * 4);
                stop_rect = stop_rect.bottom_shifted(self.speed_rate * 4);
            }
            Direction::Up => {
                self_rect = self_rect.top_shifted(SPEED_RATE * 4);
                stop_rect = stop_rect.top_shifted(self.speed_rate * 4);
            }
            Direction::Left => {
                self_rect = self_rect.left_shifted(SPEED_RATE * 4);
                stop_rect = stop_rect.left_shifted(self.speed_rate * 4);
            }
            Direction::Right => {
                self_rect = self_rect.right_shifted(SPEED_RATE * 4);
                stop_rect = stop_rect.right_shifted(self.speed_rate * 4);
            }
        };
        // println!("2 : {:?}",self_rect);
        let mut check = 0;
        for index in 0..tab_cars.len() {
            if self_index == index {
                continue;
            }
            let car = tab_cars[index].clone();
            // match self_rect.intersection(car.collider) {
            // Some(_) => return false,
            // None => continue,
            // }
            // if self_rect.has_intersection(car.collider) {
            if self_rect.has_intersection(car.collider) && check < 1 {
                check = 1;
            }
            if stop_rect.has_intersection(car.collider) && check < 2 {
                check = 2;
            }
        }
        // if check > 1 && display {
        //     canvas.set_draw_color(Color::RED);
        //     canvas.draw_rect(self_rect).unwrap();
        // } else if display {
        //     canvas.set_draw_color(Color::GREEN);
        //     canvas.draw_rect(self_rect).unwrap();
        //     if self.turn {
        //         self.speed_rate = SPEED_RATE * 2;
        //     }
        // }
        if display {
            match check {
                0 => {
                    canvas.set_draw_color(Color::GREEN);
                    if self.turn {
                        self.speed_rate = SPEED_RATE * 2;
                    } else {
                        self.speed_rate = SPEED_RATE;
                    }
                }
                1 => {
                    canvas.set_draw_color(Color::YELLOW);
                    self.speed_rate = SPEED_RATE * 2 / 5;
                }
                2 => {
                    canvas.set_draw_color(Color::RED);
                    // self.speed_rate = 0;
                }
                _ => {}
            }
            canvas.draw_rect(self_rect).unwrap();
            canvas.set_draw_color(Color::BLACK);
            canvas.draw_rect(stop_rect).unwrap();
        }
        return check < 2;
    }

    pub fn tick(
        &mut self,
        canvas: &mut Canvas<Window>,
        tab_cars: &Vec<Cars>,
        index: usize,
        tab_texture: &HashMap<CarColor, Texture>,
    ) -> bool {
        // if index == 0 {
        //     self.speed_rate = SPEED_RATE * 2;
        // } else {
        //     self.speed_rate = SPEED_RATE / 2;
        // }
        if self.check_colision(tab_cars, index, canvas, true) {
            // if true {
            match self.direction {
                Direction::Down => {
                    // let check = fire_road.get(&Direction::Down).unwrap();
                    // if check.color == FireColor::Red && self.collider.y == check.rect.y {
                    // } else {
                    self.collider = self.collider.bottom_shifted(self.speed_rate);
                    // }
                }
                Direction::Up => {
                    // let check = fire_road.get(&Direction::Up).unwrap();
                    // if check.color == FireColor::Red && self.collider.y == check.rect.y {
                    // } else {
                    self.collider = self.collider.top_shifted(self.speed_rate);
                    // }
                }
                Direction::Right => {
                    // let check = fire_road.get(&Direction::Right).unwrap();
                    // if check.color == FireColor::Red && self.collider.x == check.rect.x {
                    // } else {
                    self.collider = self.collider.right_shifted(self.speed_rate);
                    // }
                }
                Direction::Left => {
                    // let check = fire_road.get(&Direction::Left).unwrap();
                    // if check.color == FireColor::Red && self.collider.x == check.rect.x {
                    // } else {
                    self.collider = self.collider.left_shifted(self.speed_rate);
                    // }
                }
            };
            if !self.turn {
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
        }

        let canva_rect = Rect::new(
            DRAW.0 - (SCALE) as i32,
            DRAW.1 - (SCALE) as i32,
            DRAW.2 + SCALE,
            DRAW.3 + SCALE,
        );
        if canva_rect.intersection(self.collider).is_none() {
            false
        } else {
            self.draw(canvas, tab_texture);
            true
        }
    }

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

        // collide

        canvas.set_draw_color(Color::BLUE);
        canvas.draw_rect(self.collider).unwrap();
    }
}

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
