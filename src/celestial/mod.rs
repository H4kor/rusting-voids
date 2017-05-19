use std;

#[cfg(test)]
mod test;

pub trait Body {
    fn mass(&self) -> f64;
    fn radius(&self) -> f64;
    
    fn volume(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius().powf(3.0) / 3.0
    }    

    fn density(&self) -> f64 {
        self.mass() / self.volume()
    }
}


pub struct Star {
    pub _mass: f64,
    pub _radius: f64,
    pub lumosity: f32
}

impl Body for Star {
    fn mass(&self) -> f64 {
        self._mass
    }

    fn radius(&self) -> f64 {
        self._radius
    }
}


pub struct Planet {
    pub _mass: f64,
    pub _radius: f64,
}

impl Body for Planet {
    fn mass(&self) -> f64 {
        self._mass
    }

    fn radius(&self) -> f64 {
        self._radius
    }
}

