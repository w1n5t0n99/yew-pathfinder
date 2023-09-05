use std::rc::Rc;

use pathfinding::prelude::dijkstra;
use yewdux::store::Store;

use crate::board::{Board, Pos};


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

    pub fn find_shortest_path(&self, board: Rc<Board>) -> Option<(Vec<Pos>, Vec<Pos>)> {
        let mut searched_cells = Vec::new();

        let shortest_path: Option<(Vec<Pos>, u32)> = dijkstra(
            &board.start_pos,
            |p| {searched_cells.push(p.clone()); board.get_successors(*p)}.into_iter().map(|p| (p,1_u32)).collect::<Vec<(Pos, u32)>>(),
            |p| board.is_goal(*p)
        );

        shortest_path.map(|sp| (sp.0, searched_cells))
    }
}


impl Default for Pathfinder {
    fn default() -> Self {
        Pathfinder::new()
    }
}

