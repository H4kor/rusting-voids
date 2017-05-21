extern crate termion;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std;
use std::io::{self, Read, Write};
use std::iter::Iterator;

use game::display::Display;



pub struct Looper<R, W: Write> {
    pub disp: Display<W>,
    pub stdin: R
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Looper<R, W> {

    pub fn start(&mut self) {
        loop {
            self.disp.cursor_to_input();
            let b = self.stdin.next().unwrap().unwrap();
            use termion::event::Key::*;
            match b {
                Char('q') => return,
                _ => {} 
            }
            self.disp.clear();
            self.disp.render_border();
            self.disp.flush();
        }
    }
}

