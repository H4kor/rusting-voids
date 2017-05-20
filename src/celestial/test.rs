use std;

use celestial::bodies::{Body,Star,Planet};
use celestial::starsystem::{OrbitData, Orbiter, StarSystem};


#[test]
fn test_volume() {
    let b1 = Star{_mass: 1.0, _radius: 1.0, lumosity: 1.0};
    let b2 = Planet{_mass: 2.0, _radius: 2.0};
    assert!(b1.volume() == 4.0 * std::f64::consts::PI / 3.0);
    assert!(b2.volume() == 4.0 * 8.0 * std::f64::consts::PI / 3.0);
}

#[test]
fn test_orbits() {
    let mut sys = StarSystem::new(Planet{_mass: 10.0, _radius: 4.0});
    assert!(sys.getMainBody().mass() == 10.0);
    sys.getRootOrbit().addBody(
        Planet{_mass: 4.0, _radius: 2.0},
        OrbitData::Bound{e: 1.0, a: 1.0, i: 1.0}
    );
    assert!(sys.getRootOrbit().getSat(0).getBody().mass() == 4.0);
}
