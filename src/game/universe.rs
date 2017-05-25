extern crate rand;

use std::vec::Vec;
use celestial::bodies::{Star, Planet};
use celestial::starsystem::{OrbitData, StarSystem};

pub struct Universe {
    systems: Vec<StarSystem>
}

impl Universe {
    pub fn generate() -> Universe {
        let mut u = Universe{
            systems: vec![]
        };
        // generate x stars
        for i in 1..100 {
            let mut s = Star::new();
            // generate random number of planets
            let r = rand::random::<u16>() % 10;
            let mut ps = vec![];
            for j in 1..r {
                let p = Planet::generate_for(&s);
                let o = OrbitData::generate_for(&s, &p);
                ps.push((p,o));
            }
            
            let mut sys = StarSystem::new(s);
            for (p, o) in ps {
                sys.getRootOrbit().addBody(p,o);
            }
            u.systems.push(sys);
        }
        
        // return Universe
        u
    }
    
    pub fn numSystems(&self) -> usize {
        self.systems.len()
    }

    pub fn getSystem(&mut self, i: usize) -> &mut StarSystem {
        self.systems.get_mut(i).unwrap()
    }
}
