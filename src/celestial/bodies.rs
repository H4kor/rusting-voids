use std;

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

/*
 * Star
 */
pub struct Star {
    pub _mass: f64,
    pub _radius: f64,
    pub lumosity: f32
}

impl Star {
    //c'tor
    pub fn new() -> Star {
        let m = 100.0;
        let r = 30.0;
        let l = 1.0;
        Star{ _mass: m, _radius: r, lumosity: l}
    }
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

impl Planet {
    //c'tor
    pub fn new() -> Planet {
        let m = 13.0;
        let r = 2.4;
        Planet{_mass: m, _radius: r}
    }
}

impl Body for Planet {
    fn mass(&self) -> f64 {
        self._mass
    }

    fn radius(&self) -> f64 {
        self._radius
    }
}

