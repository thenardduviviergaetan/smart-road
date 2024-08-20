use sdl2::rect::Rect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gradian {
    // pub good_rect: Rect,
    pub warning_rect: Rect,
    pub stop_rect: Rect,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Collider {
    pub left: Gradian,
    pub center: Gradian,
    pub right: Gradian,
}

