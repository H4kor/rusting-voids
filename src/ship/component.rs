
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

    /// executes temporal actions 
    fn timestep(&mut self, t: &Time);
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

    fn timestep(&mut self, t: &Time) {

    }
}
