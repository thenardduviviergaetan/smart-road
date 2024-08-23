use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::video::*;
use std::path::Path;
use std::time::Duration;

use super::stats::Stats;

pub fn display(
    event_pump: &mut sdl2::EventPump,
    title: &str,
    width: u32,
    height: u32,
    stats: Stats,
) {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window(title, width, height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font_path: &Path = Path::new(&"assets/fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    canvas.set_draw_color(Color::GRAY);
    canvas.clear();
    
    let tab = [
        format!("nombre de voiture: {}", stats.nb_car),
        format!("vitesse max: {} pixels/s", round(stats.vmax)),
        format!("vitesse min: {} pixels/s", round(stats.vmin)),
        format!("temps max: {}s", round(stats.tmax)),
        format!("temps min: {}s", round(stats.tmin)),
    ];
    
    let mut target = Rect::new(0, 0, width, height / 5);
    for index in 0..tab.len() {
        let surface = font.render(&tab[index]).blended(Color::WHITE).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        canvas.copy(&texture, None, Some(target)).unwrap();
        target.y += (height / 5) as i32;
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            };
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 24));
    }
}

fn round(nb: f32) -> f32 {
    (nb * 100.0).round() / 100.0
}
