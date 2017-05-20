extern crate termion;
mod celestial;
mod game;

use std::io::{self, Read, Write};
use termion::{color, style, clear};
use termion::raw::IntoRawMode;
use celestial::bodies::{Body, Star, Planet};
use celestial::starsystem::OrbitData;
use game::Display;


fn print_val(text: &str, val: &f64) {
    let color = color::Fg(color::Rgb(1,0,0));
    let reset = color::Fg(color::Reset);
    println!("{}:{}{}{}", text, color, val, reset); 
}

fn init<W: Write, R: Read>(mut stdout: W, stdin: R, width: u16, height: u16) {
    //clear the screen
    let mut dis = Display::new(stdin, stdout, width, height);
    dis.render_border();
}

fn main() {
    // Get and lock the stdios.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    
    // We go to raw mode 
    let mut stdout = stdout.into_raw_mode().unwrap();
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w );
    let termheight = termsize.map(|(_,h)| h );

    init(stdout, stdin, termwidth.unwrap(), termheight.unwrap());

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
    
    //let sun = sys.getRootOrbit();
    //print_val("Sun Mass", &sun.getBody().mass());
    //let pl = sun.getSat(0).getBody();
    //print_val("Planet Mass", &pl.mass());

}
