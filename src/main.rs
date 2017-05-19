extern crate termion;
mod celestial;

use termion::{color, style};

use celestial::Body;

fn main() {
    let s = celestial::Star{ _mass: 100., _radius: 34.0, lumosity: 0.64 };
    println!("Mass: {}{:?}{}", color::Fg(color::Red), s.mass(), color::Fg(color::Reset));
    println!("Radius: {}{:?}{}", color::Fg(color::Red), s.radius(), color::Fg(color::Reset));
    println!("Lumosity: {}{:?}{}", color::Fg(color::Red), s.lumosity, color::Fg(color::Reset));
    println!("Volume: {}{:?}{}", color::Fg(color::Red), s.volume(), color::Fg(color::Reset));
    println!("Density: {}{:?}{}", color::Fg(color::Red), s.density(), color::Fg(color::Reset));
}
