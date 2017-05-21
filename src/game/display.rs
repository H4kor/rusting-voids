extern crate termion;

use std::io::{Read, Write};
use termion::{color, style, cursor, clear};

pub struct Display<W: Write> {
    pub o: W,
    pub width: u16,
    pub height: u16
}

impl<W: Write> Display<W> {
    pub fn new(mut o: W, width: u16, height: u16) -> Display<W>{
        
        write!(o, "{}", clear::All).unwrap();
        Display{
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

    pub fn cursor_to_input(&mut self) {
        let x = 1;
        let y = self.height - 2;
        self.goto(x,y);
    }

    pub fn clear(&mut self) {
        write!(self.o, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
    }

    pub fn flush(&mut self) {
        self.cursor_to_input();
        self.o.flush().unwrap();
    }

    pub fn render_border(&mut self) {
        //place at the bottom
        let nBottom = 4;

        for x in 1..(self.width) {
            //upper line
            self.place("═", x, 1);

            //lower line
            let h = self.height - nBottom;
            self.place("═", x, h);
        }
        
        for y in 1..(self.height - nBottom) {
            self.place("║", 1, y);
            let w = self.width;
            self.place("║", w, y);
        }
        let w = self.width;
        let h = self.height - nBottom;

        self.place("╔", 1, 1);
        self.place("╗", w, 1);
        self.place("╚", 1, h);
        self.place("╝", w, h);

        self.goto(1,1);
    }
}
