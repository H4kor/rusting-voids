extern crate rand;

pub mod time;

use units::masses::M;
use units::lens::L;
use units::UnitValue;

/// returns uniformly sampled random value between `min` and `max`
pub fn random(min: f64, max: f64) -> f64 {
    assert!(min < max);
    let diff = max - min;
    diff * (rand::random::<f64>() % 1.0) + min
}

/// returns random mass between `min` and `max`
pub fn randomMass(min: M, max: M) -> M {
    //assert!(min < max);
    let cvt = min.cvt_to(*max.unit());
    let v = random(cvt.v, max.v);
    v * max.u
}

/// returns random length between `min` and `max`
pub fn randomLength(min: L, max: L) -> L {
    //assert!(min < max);
    let cvt = min.cvt_to(*max.unit());
    let v = random(cvt.v, max.v);
    v * max.u
}
