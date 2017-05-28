
use std;
use std::string::String;

use utils::time::Time;

use ship::inventory::Inventory;


pub trait ShipComponent {
    /// returns a one line description of the component
    fn short_desc(&self) -> String;

    /// return the influence on the ships power system
    /// negative -> drains energy
    /// positive -> produces energy
    fn power_mod(&self) -> f64;

    /// changes power mode of the device the power supply
    fn toggle_power(&mut self) {

    }
    
    fn timestep(&mut self, t: &Time) {
        self._timestep(t);
    }

    /// executes temporal actions 
    /// implement this for each component
    /// will be executed by `timestep`
    fn _timestep(&mut self, t: &Time);
}

struct CargoBay {
    inv: Inventory,
}

impl ShipComponent for CargoBay {
    
    fn short_desc(&self) -> String {
        format!("CargoBay")
    }

    fn power_mod(&self) -> f64 {
        0.0
    }

    fn _timestep(&mut self, t: &Time) {

    }
}

/// Allows attaching further ShipComponent
pub struct Connector {
    pub max: usize,
    pub comps: Vec<Box<ShipComponent>>
}

impl Connector {
    fn addComponent<T: ShipComponent + 'static>(&mut self, c: T) {
        if self.comps.len() < self.max {
            self.comps.push(Box::new(c));
        }
    }

}

impl ShipComponent for Connector {
    fn short_desc(&self) -> String {
        format!("Connector")
    }

    fn power_mod(&self) -> f64 {
        let mut p = 0.0;
        for ref comp in &self.comps {
            p += comp.power_mod();
        }
        p 
    }

    fn _timestep(&mut self, t: &Time) {
        for ref mut comp in &mut self.comps {
            comp.timestep(t);
        }
    }

}
