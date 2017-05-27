
use std::vec::Vec;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use ship::component::ShipComponent;

use utils::time::Time;

pub struct ShipCore {
    comps: Vec<Box<ShipComponent>>,
}

impl ShipCore {
    pub fn new() -> ShipCore {
        ShipCore{comps: vec!()}
    }

    /// executes temporal actions 
    pub fn timestep(&mut self, t: &Time) {
        for i in 0..self.getNumComps() {
            let comp = self.getCompByPos(i);
            comp.timestep(t);
        }
    }

    pub fn power(&self) -> f64 {
        // basic power supply
        let mut p = 10.0;
        for comp in &self.comps {
            p += comp.as_ref().power_mod()
        }
        p
    }
    
    fn getNumComps(&self) -> usize {
        self.comps.len()
    }

    fn getCompByPos(&mut self, i: usize) -> &mut ShipComponent {
        self.comps[i].deref_mut() 
    }

}
