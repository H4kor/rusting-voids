extern crate termion;

use std::io::{Read, Write};
use termion::{color, style, cursor, clear};

pub struct Display<R: Read, W> {
    pub i: R,
    pub o: W,
    pub width: u16,
    pub height: u16
}

impl<R: Read, W: Write> Display<R,W> {
    pub fn new(i: R, mut o: W, width: u16, height: u16) -> Display<R,W>{
        
        write!(o, "{}", clear::All).unwrap();
        Display{
            i: i, 
            o: o,
            width: width,
            height: height
        }
    }

    fn goto(&mut self, x: u16, y: u16) {
        write!(self.o, "{}", cursor::Goto(x,y)).unwrap();
    }

    fn place(&mut self, text: &str, x: u16, y: u16) {
        self.goto(x,y);
        write!(self.o, "{}", text);
    }

    pub fn render_border(&mut self) {
        //place at the bottom
        let place = 4;

        for x in 1..(self.width+1) {
            //upper line
            self.place("═", x, 1);

            //lower line
            let h = self.height - place;
            self.place("═", x, h);
        }
        
        for y in 1..(self.height - place) {
            self.place("║", 1, y);
            let w = self.width;
            self.place("║", w, y);
        }
        let w = self.width;
        let h = self.height - place;

        self.place("╔", 1, 1);
        self.place("╗", w, 1);
        self.place("╚", 1, h);
        self.place("╝", w, h);

        self.goto(1,1);
    }
}
