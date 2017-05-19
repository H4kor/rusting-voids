mod celestial;

use celestial::Body;

fn main() {
    let s = celestial::Star{ _mass: 100., _radius: 34.0, lumosity: 0.64 };
    println!("Mass: {:?}", s.mass());
    println!("Radius: {:?}", s.radius());
    println!("Lumosity: {:?}", s.lumosity);
    println!("Volume: {:?}", s.volume());
    println!("Density: {:?}", s.density());
}
