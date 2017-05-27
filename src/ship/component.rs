
use std;
use std::string::String;
use utils::time::Time;

pub trait Component {
    /// returns a one line description of the component
    fn short_desc(&self) -> String;

    /// return the influence on the ships power system
    /// negative -> drains energy
    /// positive -> produces energy
    fn power_mod(&self) -> f64;

    /// executes temporal actions 
    fn timestep(&mut self, t: &Time);
}
