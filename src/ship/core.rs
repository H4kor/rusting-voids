
use std::vec::Vec;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use ship::component::{Connector, ShipComponent};

use utils::time::Time;

pub struct ShipCore {
    main_connector: Connector,
}

impl ShipCore {
    pub fn new() -> ShipCore {
        ShipCore{
            main_connector: Connector{
                max: 4,
                comps: vec!()
            }
        }
    }

    /// executes temporal actions 
    pub fn timestep(&mut self, t: &Time) {
        self.main_connector.timestep(t);
    }

    pub fn power(&self) -> f64 {
        // basic power supply
        let mut p = 10.0;
        p += self.main_connector.power_mod();
        p
    }
}
