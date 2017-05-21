extern crate termion;
mod celestial;
mod game;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;

use std::io::{self, Read, Write};

use celestial::bodies::{Body, Star, Planet};
use celestial::starsystem::OrbitData;

use game::display::Display;
use game::looper::Looper;


fn init<W: Write, R: Read>(mut stdout: W, stdin: R, width: u16, height: u16) {
    //clear the screen
    //let mut k = stdin.keys();
    //let b = k.next().unwrap().unwrap();       

    let mut looper= Looper {
        disp: Display::new(stdout, width, height),
        //disp: Display::new(stdout, 40, 20),
        stdin: stdin.keys()
    };

    looper.start();
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
    let termwidth = termsize.map(|(w,_)| w-2 );
    let termheight = termsize.map(|(_,h)| h-2 );

    //let mut k = stdin.keys();
    init(stdout, stdin, termwidth.unwrap(), termheight.unwrap());

    //let mut sys = celestial::starsystem::StarSystem::new(
    //    celestial::bodies::Star::new()
    //);
    
    // add a planet in scope to free pl afterwards
    //{
    //    let pl = sys.getRootOrbit().addBody(
    //        celestial::bodies::Planet::new(),
    //        celestial::starsystem::OrbitData::Bound{ a: 1., e: 1., i: 1.}
    //    );
    //}
    
    //let sun = sys.getRootOrbit();
    //print_val("Sun Mass", &sun.getBody().mass());
    //let pl = sun.getSat(0).getBody();
    //print_val("Planet Mass", &pl.mass());

}
