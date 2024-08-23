use super::{
    cars::Cars,
    constante::{SCALE, WIDTH},
    impl_enum::CarTurn,
};

#[derive(Debug)]
pub struct Stats {
    pub nb_car: u32,
    pub vmax: f32,
    pub vmin: f32,
    pub tmax: f32,
    pub tmin: f32,
}

impl Stats {
    // init stats object
    pub fn new() -> Self {
        Self {
            nb_car: 0,
            vmax: 0.0,
            vmin: f32::MAX,
            tmax: 0.0,
            tmin: f32::MAX,
        }
    }

    // function which updates the stats object
    pub fn actualise(&mut self, cars: Cars) {
        self.nb_car += 1;
        let vitesse = match cars.direction_turn {
            CarTurn::Left => (WIDTH + 2 * SCALE) as f32 / cars.instant.elapsed().as_secs_f32(),
            CarTurn::Right => (WIDTH - 4 * SCALE) as f32 / cars.instant.elapsed().as_secs_f32(),
            CarTurn::None => WIDTH as f32 / cars.instant.elapsed().as_secs_f32(),
        };
        if vitesse > self.vmax{
            self.vmax = vitesse;
        }
        if self.vmin > vitesse {
            self.vmin = vitesse;
        }
        let temp = cars.instant.elapsed().as_secs_f32();
        if self.tmax < temp {
            self.tmax = temp;
        }
        if self.tmin > temp {
            self.tmin = temp;
        }
    }
}