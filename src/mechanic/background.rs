extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::sdl2::image::LoadTexture;
use sdl2::render::*;
use sdl2::video::*;

use super::constante::DRAW;

pub fn background(canvas: &mut Canvas<Window>,texture_creator: &TextureCreator<WindowContext>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    let texture = texture_creator
        .load_texture(format!("./assets/road_800x800.jpg"))
        .unwrap();
    let target = Rect::from(DRAW);
    canvas
        .copy_ex(&texture, None, Some(target), 0.0, None, false, false)
        .unwrap();
}