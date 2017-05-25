use std;

use celestial::bodies::{Body,Star,Planet};
use celestial::starsystem::{OrbitData, Orbiter, StarSystem};
use units::UnitValue;
use units::masses::MU;
use units::lens::LU;

#[test]
fn test_volume() {
 //TODO: reimplement when volume measurements are available
 //   let b1 = Star{_mass: 1.0, _radius: 1.0, lumosity: 1.0};
 //   let b2 = Planet{_mass: 2.0, _radius: 2.0};
 //   assert!(b1.volume() == 4.0 * std::f64::consts::PI / 3.0);
 //   assert!(b2.volume() == 4.0 * 8.0 * std::f64::consts::PI / 3.0);
}

#[test]
fn test_orbits() {
    let mut sys = StarSystem::new(Planet{_mass: 10.0*MU::kg, _radius: 4.0*LU::km});
    assert!(sys.getMainBody().mass().v == 10.0);
    sys.getRootOrbit().addBody(
        Planet{_mass: 4.0*MU::kg, _radius: 2.0*LU::km},
        OrbitData::Bound{e: 1.0, a: 1.0, i: 1.0}
    );
    assert!(sys.getRootOrbit().getSat(0).getBody().mass().v == 4.0);
}
