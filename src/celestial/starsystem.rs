use std;

use celestial::bodies::Body;


#[derive(PartialEq)]
pub enum OrbitData {
    NotBound,
    Bound{ 
        a: f64,         //Semi-Major Axis
        e: f64,         //Eccentricity
        i: f64,         //inclination
        //body: Orbiter   //body being orbited
    }
}


pub struct Orbiter {
    /// the body orbiting
    pub body: Box<Body>,
    /// properties of the orbit
    pub data: OrbitData,
    /// satellite objects
    pub sats: std::vec::Vec<Box<Orbiter>>
}

pub struct StarSystem {
    pub main_body: Orbiter
}

impl StarSystem {
   
    /**
     * Create a empty StarSystem with T-body as main_body
     */
    pub fn new(root: Box<Body>) -> StarSystem {
        StarSystem {
            main_body: Orbiter{
                body: root, 
                data: OrbitData::NotBound,
                sats: vec![]
            }
        }
    }
}

impl Orbiter {
    pub fn addBody<T: Body + 'static>(&mut self, body: T, orbit: OrbitData) 
    -> &Box<Orbiter> {       
        assert!(orbit != OrbitData::NotBound);

        let o = Orbiter{
            body: Box::new(body), 
            data: orbit,
            sats: vec![]
        };
        let boxed = Box::new(o);
        self.sats.push(boxed);
        self.sats.last().unwrap()
    }

}
