extern crate rand;

use std;
use std::string::String;

use utils::{randomMass, randomLength};

use units::UnitValue;
use units::lens::L;
use units::lens::LU::{km, lsec};
use units::masses::M;
use units::masses::MU::{tons, ktons, mtons, earth_mass, jupiter_mass, sun_mass};

pub trait Body {
    fn mass(&self) -> M;
    fn radius(&self) -> L;
    
    fn short_desc(&self) -> String; 
    ///TODO: convert to volume type
    fn volume(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius().v.powf(3.0) / 3.0
    }    
    ///TODO: convert to density type
    fn density(&self) -> f64 {
        self.mass().v / self.volume()
    }
}

/**
 * Star
 */
pub struct Star {
    pub _mass: M,
    pub _radius: L,
    pub lumosity: f32
}

impl Star {
    //c'tor
    pub fn new() -> Star {
        let r = randomLength(0.1 * 695700.0 * km, 100.0 * 695700.0 * km);
        let m = randomMass(0.1 * sun_mass, 100.0 * sun_mass);
        let l = 1.0;
        Star{ _mass: m, _radius: r, lumosity: l}
    }
}

impl Body for Star {
    fn mass(&self) -> M {
        self._mass
    }

    fn radius(&self) -> L {
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
    pub _mass: M,
    pub _radius: L,
}

impl Planet {
    //c'tor
    pub fn new() -> Planet {
        let m = 1.0 * earth_mass;
        let r = 16e5 * km;
        Planet{_mass: m, _radius: r}
    }
    
    pub fn generate_for<S: Body>(star: &S) -> Planet {
        let m = randomMass(1e-15 * star.mass(), 1e-6 * star.mass());
        //between moon and jupiter
        let r = randomLength(3390.0 * km, 69911.0 * km);
        Planet { _mass: m, _radius: r}
    }

}

impl Body for Planet {
    fn mass(&self) -> M {
        self._mass
    }

    fn radius(&self) -> L {
        self._radius
    }
    
    fn short_desc(&self) -> String {
        format!("Planet: Mass: {:.4}, Radius: {:.4}", 
            self.mass(),
            self.radius()
        )
    
    }
}

