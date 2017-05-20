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
    pub fn new<T: Body + 'static>(root: T) -> StarSystem {
        StarSystem::new_from_box(Box::new(root))
    }
    pub fn new_from_box(root: Box<Body>) -> StarSystem {
        StarSystem {
            main_body: Orbiter{
                body: root, 
                data: OrbitData::NotBound,
                sats: vec![]
            }
        }
    }  

    pub fn getMainBody(&self) -> &Body {
        self.main_body.getBody()
    }

    pub fn getRootOrbit(&mut self) -> &mut Orbiter {
        &mut self.main_body
    }

    pub fn getNumSats(&self) -> usize {
        self.main_body.getNumSats()
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

    pub fn getBody(&self) -> &Body {
        self.body.as_ref()
    }

    pub fn getNumSats(&self) -> usize {
        self.sats.len()
    }

    pub fn getSat(&self, i: usize) -> &Orbiter {
        self.sats[i].as_ref()
    }
}
