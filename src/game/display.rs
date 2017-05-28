extern crate termion;

use std::vec::Vec;
use std::io::{Read, Write};
use termion::{color, style, cursor, clear};

use celestial::starsystem::StarSystem;


pub struct DisplayListItem {
    pub item: String,
    pub subitems: Vec<DisplayListItem>,
}

impl DisplayListItem {
    pub fn add(&mut self, item: String) -> usize {
        self.subitems.push(DisplayListItem{
            item: item,
            subitems: vec!()
        });
        self.subitems.len() - 1
    }
}

pub struct DisplayList {
    pub items: Vec<DisplayListItem>,
}

impl DisplayList {
    pub fn new() -> DisplayList {
        DisplayList{items: vec!()}
    }

    pub fn add(&mut self, item: String) -> usize {
        self.items.push(DisplayListItem{
            item: item,
            subitems: vec!()
        });
        self.items.len() - 1

    }

    pub fn addTo(&mut self, k: usize, item: String) -> &mut DisplayListItem {
        self.items[k].subitems.push(DisplayListItem{
            item: item,
            subitems: vec!()
        });
        self.items[k].subitems.last_mut().unwrap()
    }
}

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

    pub fn place(&mut self, text: &str, x: u16, y: u16) {
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

    pub fn set_title(&mut self, title: &str) {
        //decorate
        let f_title = format!("╡{}   {}   {}╞", 
                              color::FBg(color::Black, color::Blue), 
                              title, 
                              color::FBg(color::Reset, color::Reset)
                             );
        //compute positions
        let len = f_title.len() as u16;
        let center: u16 = self.width/2;
        let start  = center - len/2;
        
        let start = 10;
        self.place(f_title.as_str(), start, 1);
    }

    pub fn print_list(&mut self, list: DisplayList) {
        self._print_list(list.items, 2, 3);
    }    
    fn _print_list(&mut self, list: Vec<DisplayListItem>, intend: u16, line: u16) -> u16 {
        // how much to increase with each sub list
        let inc = 4;
        let x = intend;
        let mut y = line;
        
        for item in list {
            self.place(item.item.as_str(), intend, y);
            y += 1;
            y = self._print_list(item.subitems, intend + inc, y);
        }
        
        y
    }
}
