use std;
use celestial::Body;

#[test]
fn test_volume() {
    let b1 = Body{mass: 1.0, radius: 1.0};
    let b2 = Body{mass: 2.0, radius: 2.0};
    assert!(b1.volume() == 4.0 * std::f32::consts::PI / 3.0);
    assert!(b2.volume() == 4.0 * 8.0 * std::f32::consts::PI / 3.0);
}
