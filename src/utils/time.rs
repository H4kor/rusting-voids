

pub struct Time {
    /// total time spent in the game
    pub total: f64,
    /// current time delta
    pub delta: f64,
}

impl Time {
    
    pub fn applyDelta(&mut self, d: f64) {
        self.total += d;
        self.delta = d;
    }
}
