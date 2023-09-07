use yewdux::store::Store;

use crate::board::Algorithm;



#[derive(Debug, PartialEq, Clone)]
pub enum Speed {
    Fast,
    Average,
    Slow,
}

#[derive(Clone, PartialEq, Store)]
pub struct NavState {
    pub algorithm: Algorithm,
    pub speed: Speed,
}

impl NavState {
    pub fn new() -> Self {
        NavState { 
            algorithm: Algorithm::Dijkstra,
            speed: Speed::Fast,
        }
    }

    pub fn reset(&mut self) {
        self.algorithm = Algorithm::Dijkstra;
        self.speed = Speed::Fast;
    }
}


impl Default for NavState {
    fn default() -> Self {
        NavState::new()
    }
}

