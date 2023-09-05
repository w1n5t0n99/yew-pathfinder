use yewdux::store::Store;


#[derive(Debug, PartialEq, Clone)]
pub enum Algorithm {
    Astar,
    Dijkstra,
    BreadthFirst,
    DepthFirst,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Speed {
    Fast,
    Average,
    Slow,
}

#[derive(Clone, PartialEq, Store)]
pub struct Pathfinder {
    pub algorithm: Algorithm,
    pub speed: Speed,
    pub is_pathfinding: bool,
}

impl Pathfinder {
    pub fn new() -> Self {
        Pathfinder { 
            algorithm: Algorithm::Astar,
            speed: Speed::Fast,
            is_pathfinding: false 
        }
    }

    pub fn reset(&mut self) {
        self.algorithm = Algorithm::Astar;
        self.speed = Speed::Fast;
        self.is_pathfinding = false;
    }
}


impl Default for Pathfinder {
    fn default() -> Self {
        Pathfinder::new()
    }
}

