use std;

pub struct Body {
    pub mass: f32,
    pub radius: f32
}

pub struct Star {
    pub body: Body,
    pub lumosity: f32,
}

pub struct Planet {
    pub body: Body
}

impl Body {
    pub fn volume(&self) -> f32 {
        4.0 * std::f32::consts::PI * self.radius.powf(3.0) / 3.0
    }

    pub fn density(&self) -> f32 {
        self.mass / self.volume()
    }
}
