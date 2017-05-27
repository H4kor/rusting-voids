
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use units::ConvertibleUnit;
use units::UnitValue;


/// Units used to denote masses in the game
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MU {
    mg,
    g,
    kg,
    tons,
    ktons,
    mtons,
    earth_mass,
    jupiter_mass,
    sun_mass
}

impl fmt::Display for MU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let u_str = match *self {
            MU::mg           => "Milli Gram", 
            MU::g            => "Gram",
            MU::kg           => "Kilo Gram",
            MU::tons         => "Tons",
            MU::ktons        => "Kilo Tons",
            MU::mtons        => "Mega Tons",
            MU::earth_mass   => "Planetary Masses",
            MU::jupiter_mass => "Gas Giant Masses",
            MU::sun_mass     => "Star Masses"
        };
        write!(f, "{}", u_str)
    }
}

impl ConvertibleUnit for MU{
    fn min() -> MU {
        MU::mg
    }
    fn max() -> MU {
        MU::sun_mass
    }

    fn up(&self) -> (f64, MU) {
        match *self {
            MU::mg           => (0.001, MU::g),
            MU::g            => (0.001, MU::kg),
            MU::kg           => (0.001, MU::tons),
            MU::tons         => (0.001, MU::ktons),
            MU::ktons        => (0.001, MU::mtons),
            MU::mtons        => (1.673360107095e-16,MU::earth_mass),
            MU::earth_mass   => (0.00314635457, MU::jupiter_mass),
            MU::jupiter_mass => (0.00095459925, MU::sun_mass),
            MU::sun_mass     => (1.0, MU::sun_mass),
            _ => panic!("No conversion known")
        }
    }

    fn down(&self) -> (f64, MU) {
        match *self {
            MU::mg           => (1.0, MU::mg),
            MU::g            => (1000.0, MU::mg ),
            MU::kg           => (1000.0, MU::g ),
            MU::tons         => (1000.0, MU::kg ),
            MU::ktons        => (1000.0, MU::tons ),
            MU::mtons        => (1000.0, MU::ktons ),
            MU::earth_mass   => (5.976e15, MU::mtons ),
            MU::jupiter_mass => (317.828133401, MU::earth_mass ),
            MU::sun_mass     => (1047.56001013, MU::jupiter_mass ),
            _ => panic!("No conversion known!")
        }
    }
}

#[derive(Clone, Copy)]
pub struct M{
    pub v: f64,
    pub u: MU
}

impl UnitValue<MU> for M {
    fn new(v: f64, u: MU) -> M {
        M{v: v, u: u}
    }
    fn unit(&self) -> &MU {
        &self.u
    }
    fn value(&self) -> f64 {
        self.v
    }
}

unit_arithmetic!(M);
unit_construction!(MU, M);
unit_disply!(M);
