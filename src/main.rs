extern crate termion;
mod celestial;

use termion::{color, style};
use celestial::bodies::{Body, Star, Planet};
use celestial::starsystem::OrbitData;

fn print_val(text: &str, val: &f64) {
    let color = color::Fg(color::Rgb(1,0,0));
    let reset = color::Fg(color::Reset);
    println!("{}:{}{}{}", text, color, val, reset); 
}

fn main() {
    let mut sys = celestial::starsystem::StarSystem::new(
        celestial::bodies::Star::new()
    );
    
    // add a planet in scope to free pl afterwards
    {
        let pl = sys.getRootOrbit().addBody(
            celestial::bodies::Planet::new(),
            celestial::starsystem::OrbitData::Bound{ a: 1., e: 1., i: 1.}
        );
    }
    
    let sun = sys.getRootOrbit();
    print_val("Sun Mass", &sun.getBody().mass());
    let pl = sun.getSat(0).getBody();
    print_val("Planet Mass", &pl.mass());

}
