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
    pub fn new() -> Self {
        Self {
            nb_car: 0,
            vmax: 0.0,
            vmin: f32::MAX,
            tmax: 0.0,
            tmin: f32::MAX,
        }
    }
    pub fn actualise(&mut self, cars: Cars) {
        self.nb_car += 1;
        // if cars.vmax as f32 > self.vmax{
            // self.vmax = cars.vmax.clone() as f32;
        // }
        // if self.vmin > cars.vmin as f32 {
            // self.vmin = cars.vmin as f32;
        // }
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

// Nombre maximal de véhicules ayant traversé l'intersection
// Vitesse maximale de tous les véhicules (Afficher la vitesse la plus rapide atteinte)
// Vitesse minimale de tous les véhicules (Affichage de la vitesse la plus lente atteinte)
// Temps maximum que les véhicules ont mis pour passer l'intersection (pour tous les véhicules, afficher celui qui a mis le plus de temps)
// Temps minimum que les véhicules ont mis pour passer l'intersection (pour tous les véhicules, afficher celui qui a mis le moins de temps)
// Le temps commence à compter dès que le véhicule est détecté par l' algorithme d'intersection intelligent jusqu'à la fin de l'intersection,
// c'est-à-dire à ce moment-là, le véhicule est retiré de la toile.
// Les accidents évités de justesse se produisent lorsque deux véhicules se croisent en violant la distance de sécurité.
