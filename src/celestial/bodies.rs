extern crate rand;

use std;
use std::string::String;

pub trait Body {
    fn mass(&self) -> f64;
    fn radius(&self) -> f64;
    
    fn short_desc(&self) -> String; 

    fn volume(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius().powf(3.0) / 3.0
    }    

    fn density(&self) -> f64 {
        self.mass() / self.volume()
    }
}

/**
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

    fn short_desc(&self) -> String {
        format!("Star: Mass: {:.4}, Radius: {:.4}, Brightness: {:.4}", 
                self.mass(),
                self.radius(),
                self.lumosity)
    }
}

/**
 * Planet
 */
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
    
    pub fn generate_for<S: Body>(star: &S) -> Planet {
        let m = rand::random::<f64>() % (star.mass() / 100.0);
        let r = rand::random::<f64>() % m;
        
        Planet { _mass: m, _radius: r}
    }

}

impl Body for Planet {
    fn mass(&self) -> f64 {
        self._mass
    }

    fn radius(&self) -> f64 {
        self._radius
    }
    
    fn short_desc(&self) -> String {
        format!("Planet: Mass: {:.4}, Radius: {:.4}", 
            self.mass(),
            self.radius()
        )
    
    }
}

