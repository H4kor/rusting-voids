extern crate termion;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std;
use std::cmp;
use std::io::{self, Read, Write};
use std::iter::Iterator;

use utils::time::Time;

use game::display::{Display, DisplayList, DisplayListItem};
use game::universe::Universe;

use ship::core::ShipCore;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LooperMode {
    SystemView,
    ShipView,
    Exit,
}

pub struct Looper<R, W: Write> {
    pub disp: Display<W>,
    pub uni: Universe,
    pub stdin: R,
    pub disp_system: usize,
    pub ship: ShipCore,
    pub time: Time,
    pub mode: LooperMode,
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Looper<R, W> {

    pub fn start(&mut self) {
        loop {
            // update time
            self.time.applyDelta(1.0);
            
            // get input and check if game should stop
            self.mode = match self.mode {
                LooperMode::SystemView => self.inputSystemView(),
                LooperMode::ShipView   => self.inputShipView(),
                _ => panic!("No Mode Selected")
            };
            print!("{:?}", self.mode);
            // stop the game if needed
            if self.mode == LooperMode::Exit {
                return
            }
            
            // update stuff
            self.ship.timestep(&self.time);

            // draw stuff
            match self.mode {
                LooperMode::SystemView => self.viewSystemView(),
                LooperMode::ShipView   => self.viewShipView(),
                _ => panic!("No Mode Selected")
            }
            self.disp.flush();
        }
    }

    fn inputShipView(&mut self) -> LooperMode {
        println!("Ship View");
        self.disp.cursor_to_input();
        let b = self.stdin.next().unwrap().unwrap();
        use termion::event::Key::*;
        match b {
            Char('q') => return LooperMode::Exit,
            Char('m') => return LooperMode::SystemView,
            _ => {} 
        }
        return LooperMode::ShipView
    }

    fn inputSystemView(&mut self) -> LooperMode {
        println!("System View");
        self.disp.cursor_to_input();
        let b = self.stdin.next().unwrap().unwrap();
        use termion::event::Key::*;
        match b {
            Char('q') => return LooperMode::Exit,
            Char('c') => return LooperMode::ShipView,
            Char('a') => self.disp_system = cmp::max(1, self.disp_system) - 1,
            Char('d') => self.disp_system = cmp::min(self.uni.numSystems() - 1, self.disp_system + 1),
            _ => {} 
        }
        return LooperMode::SystemView
    }

    fn viewSystemView(&mut self) {
        self.disp.clear();
        self.disp.render_border();
        self.disp.set_title(format!("System: {}", self.disp_system).as_str());
        
        let sys = self.uni.getSystem(self.disp_system);
        let mut list = DisplayList::new();
        let k = list.add(sys.getMainBody().short_desc());
        for i in 0..sys.getNumSats() {
            let body = sys.getRootOrbit().getSat(i).getBody();
            let mut sub = list.addTo(k, body.short_desc());
            sub.add(format!("Mass: {}", body.mass()));    
            sub.add(format!("Radius: {}", body.radius()));    
        }
        self.disp.print_list(list);

        //self.disp.list_system(self.uni.getSystem(self.disp_system));
     }

    fn viewShipView(&mut self) {
        self.disp.clear();
        self.disp.render_border();
        self.disp.set_title("Ship Details"); 
        self.disp.place("The ship is completely dark.", 3, 3);    
    }
}
