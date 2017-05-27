
use std::vec::Vec;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use ship::component::Component;

use utils::time::Time;

pub struct ShipCore {
    comps: Vec<Box<Component>>,
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

    fn getNumComps(&self) -> usize {
        self.comps.len()
    }

    fn getCompByPos(&mut self, i: usize) -> &mut Component {
        self.comps[i].deref_mut() 
    }
}
