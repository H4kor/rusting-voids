
use std::vec::Vec;

use units::masses::{M, MU};

pub trait Item {
    fn mass(&self) -> M;
    fn volume(&self) -> f64;
}

pub struct Inventory {
    pub items: Vec<Box<Item>>,
    pub mass_cap: M,
    pub volume_cap: f64,
}

impl Inventory {

    fn mass(&self) -> M {
        let mut m = 0.0 * MU::mg;
        for item in &self.items {
            m = m + item.as_ref().mass();
        }
        m
    }

    fn volume(&self) -> f64 {
        let mut m = 0.0;
        for item in &self.items {
            m += item.as_ref().volume();
        }
        m
    }

}
