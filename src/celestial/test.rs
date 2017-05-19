use std;

use celestial::Body;
use celestial::Star;
use celestial::Planet;

#[test]
fn test_volume() {
    let b1 = Star{_mass: 1.0, _radius: 1.0, lumosity: 1.0};
    let b2 = Planet{_mass: 2.0, _radius: 2.0};
    assert!(b1.volume() == 4.0 * std::f64::consts::PI / 3.0);
    assert!(b2.volume() == 4.0 * 8.0 * std::f64::consts::PI / 3.0);
}
