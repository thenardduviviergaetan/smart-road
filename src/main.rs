mod mechanic;
use mc::impl_enum::Direction;
use mechanic as mc;
use mechanic::cars::{self, Cars};
use mechanic::constante::{RECT_CROSS, THROTTLE_DURATION};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
pub fn main() {
    // creation Window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "road_intersection",
            mc::constante::WIDTH,
            mc::constante::HEIGHT,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().unwrap();

    // texture
    let texture_creator = canvas.texture_creator();
    let hash_map_texture_car = cars::init_texture(&texture_creator);
    mc::background::background(&mut canvas, &texture_creator);
    let mut tab_cars: Vec<Cars> = Vec::new();
    let mut temp_cars: Vec<Cars> = Vec::new();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut frezze = false;
    let mut key_input = Instant::now();
    let throttle = Duration::new(0, THROTTLE_DURATION);
    'running: loop {
        i = (i + 1) % 255;
        mc::background::background(&mut canvas, &texture_creator);
        for mut c in temp_cars.drain(..).collect::<Vec<Cars>>() {
            if c.check_colision(&tab_cars, usize::MAX, &mut canvas, false) && tab_cars.len() < 17 {
                // if c.check_colision(&tab_cars, usize::MAX, &mut canvas, false) {
                tab_cars.push(c)
            } else {
                temp_cars.push(c);
            }
        }
        if !frezze {
            let mut temp_tab: Vec<Cars> = Vec::new();
            for index in 0..tab_cars.len() {
                let mut cars = tab_cars[index].clone();
                if cars.tick(&mut canvas, &tab_cars, index, &hash_map_texture_car) {
                    temp_tab.push(cars);
                }
            }
            tab_cars = temp_tab;
        } else {
            for c in tab_cars.clone() {
                c.draw(&mut canvas, &hash_map_texture_car);
            }
        }
        for event in event_pump.poll_iter() {
            if throttle.as_nanos() <= key_input.elapsed().as_nanos() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if temp_cars.len() <= 1 {
                            match key {
                                Keycode::UP => temp_cars.push(Cars::random(Some(Direction::Up))),
                                Keycode::Right => {
                                    temp_cars.push(Cars::random(Some(Direction::Right)))
                                }
                                Keycode::Left => {
                                    temp_cars.push(Cars::random(Some(Direction::Left)))
                                }
                                Keycode::Down => {
                                    temp_cars.push(Cars::random(Some(Direction::Down)))
                                }
                                Keycode::R => temp_cars.push(Cars::random(None)),
                                Keycode::Space => frezze = !frezze,
                                Keycode::Asterisk => tab_cars = Vec::new(),
                                _ => {}
                            }
                        }
                        // println!("{key}");
                        // println!(
                        //     "throttle: {:?}, key_input: {:?}",
                        //     throttle.as_nanos(),
                        //     key_input.elapsed().as_nanos()
                        // );
                        key_input = Instant::now();
                    }
                    _ => {}
                }
            }
        }
        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(Rect::from(RECT_CROSS)).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, mc::constante::FRAME));
    }
}
