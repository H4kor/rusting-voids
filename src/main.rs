mod celestial;

fn main() {
    let s = celestial::Star{ body:celestial::Body{mass: 100., radius: 34.0}, lumosity: 0.64 };
    println!("Mass: {:?}", s.body.mass);
    println!("Radius: {:?}", s.body.radius);
    println!("Lumosity: {:?}", s.lumosity);
    println!("Volume: {:?}", s.body.volume());
    println!("Density: {:?}", s.body.density());
}
