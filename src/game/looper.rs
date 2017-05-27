extern crate termion;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std;
use std::cmp;
use std::io::{self, Read, Write};
use std::iter::Iterator;

use utils::time::Time;

use game::display::Display;
use game::universe::Universe;

use ship::core::ShipCore;

pub struct Looper<R, W: Write> {
    pub disp: Display<W>,
    pub uni: Universe,
    pub stdin: R,
    pub disp_system: usize,
    pub ship: ShipCore,
    pub time: Time,
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Looper<R, W> {

    pub fn start(&mut self) {
        loop {
            // update time
            self.time.applyDelta(1.0);
            
            // get input
            self.disp.cursor_to_input();
            let b = self.stdin.next().unwrap().unwrap();
            use termion::event::Key::*;
            match b {
                Char('q') => return,
                Char('a') => self.disp_system = cmp::max(1, self.disp_system) - 1,
                Char('d') => self.disp_system = cmp::min(self.uni.numSystems() - 1, self.disp_system + 1),
                _ => {} 
            }
            
            // update stuff
            self.ship.timestep(&self.time);

            // draw stuff
            self.disp.clear();
            self.disp.render_border();
            if self.disp_system >= 0 && self.disp_system <= self.uni.numSystems() {
                self.disp.place(format!("System: {}", self.disp_system).as_str(), 1, 1);
                self.disp.list_system(self.uni.getSystem(self.disp_system));
            }
            self.disp.flush();
        }
    }
}

