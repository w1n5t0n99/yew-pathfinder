use pathfinding::{prelude::{dijkstra, astar, bfs, dfs, fringe, iddfs}};
use yewdux::store::Store;
use std::collections::HashSet;

use crate::cell::Cell;


const CELL_LENGTH: u32 = 28;
const MAX_CELLS_PER_ROW: u32 = 50;
const MAX_CELLS_PER_COLUMN: u32 = 50;
const MIN_CELLS_PER_ROW: u32 = 10;
const MIN_CELLS_PER_COLUMN: u32 = 10;

#[derive(Debug, PartialEq, Clone)]
pub enum Algorithm {
    Astar,
    Dijkstra,
    BreadthFirst,
    Fringe,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub u16, pub u16);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellType {
    Empty,
    Wall,
    Start,
    End,
    ShortestPath(u16),
    Visited(u16),
}

#[derive(Clone, PartialEq, Store)]
pub struct Board {
    pub cells_per_row: u32,
    pub cells_per_column: u32,
    pub cells: Vec<CellType>,
    pub allow_drawing: bool,
    pub start_pos: Pos,
    pub end_pos: Pos,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        // calc number of cells
        let cells_per_row = (width / CELL_LENGTH).clamp(MIN_CELLS_PER_ROW, MAX_CELLS_PER_ROW);
        let cells_per_column = (height/ CELL_LENGTH).clamp(MIN_CELLS_PER_COLUMN, MAX_CELLS_PER_COLUMN);

        let mut cells = Vec::with_capacity((MAX_CELLS_PER_ROW * MAX_CELLS_PER_COLUMN) as usize);
        cells.resize((cells_per_row * cells_per_column) as usize, CellType::Empty);
        
        // calc start and end position
        let y = cells_per_column / 2;
        let sx = 2 as u32;
        let ex = cells_per_row - 3;

        cells[((cells_per_row * y) + sx) as usize] = CellType::Start;
        cells[((cells_per_row * y) + ex) as usize] = CellType::End;

        Board {
            cells_per_row,
            cells_per_column,
            cells,
            allow_drawing: false,
            start_pos: Pos(sx as u16, y as u16),
            end_pos: Pos(ex as u16, y as u16),
        }
    }

    pub fn reset(&mut self, width: u32, height: u32) {
        // calc number of cells
        let cells_per_row = (width / CELL_LENGTH).clamp(MIN_CELLS_PER_ROW, MAX_CELLS_PER_ROW);
        let cells_per_column = (height/ CELL_LENGTH).clamp(MIN_CELLS_PER_COLUMN, MAX_CELLS_PER_COLUMN);

        // reset the cells
        self.cells_per_row = cells_per_row;
        self.cells_per_column = cells_per_column;
        self.cells.clear();
        self.cells.resize((cells_per_row * cells_per_column) as usize, CellType::Empty);

        // calc start and end position
        let y = cells_per_column / 2;
        let sx = 2 as u32;
        let ex = cells_per_row - 3;

        self.start_pos = Pos(sx as u16, y as u16);
        self.end_pos = Pos(ex as u16, y as u16);

        self.cells[((cells_per_row * y) + sx) as usize] = CellType::Start;
        self.cells[((cells_per_row * y) + ex) as usize] = CellType::End;
    }

    pub fn clear_board(&mut self) {
        for c in self.cells.iter_mut() {
            if *c != CellType::Start && *c != CellType::End {
                *c = CellType::Empty;
            }
        }
    }

    pub fn clear_path(&mut self) {
        for c in self.cells.iter_mut() {
            match *c {
                CellType::Visited(_) | CellType::ShortestPath(_) => {
                    *c = CellType::Empty;
                }
                _=> { }
            }
        }
    }

    pub fn clear_cell_by_index(&mut self, index: usize) {
        self.cells[index] = CellType::Empty;
    }

    pub fn set_wall_by_index(&mut self, index: usize) {
        if self.cells[index] == CellType::Empty {
            self.cells[index] = CellType::Wall;
        }
    }

    pub fn set_cell_by_pathfinding(&mut self, visited: &[Pos], shortest_path: &[Pos], delay: u16 ) {
        // set visited cells
        for (i,c) in visited.iter().enumerate() {
            let index = ((self.cells_per_row * c.1 as u32) + c.0 as u32) as usize; 
            if self.cells[index] == CellType::Empty {
                self.cells[index] = CellType::Visited((i as u16)*delay);
            }
        }

        // set shortest path cells
        for c in shortest_path.iter() {
            let index = ((self.cells_per_row * c.1 as u32) + c.0 as u32) as usize; 
            if let CellType::Visited(delay) = self.cells[index] {
                self.cells[index] = CellType::ShortestPath(delay);
            }
        }
    }

    pub fn get_cell_by_pos(&self, pos: Pos) -> CellType {
        self.cells[((self.cells_per_row * pos.1 as u32) + pos.0 as u32) as usize]
    }

    pub fn get_successors(&self, pos: Pos) -> Vec<Pos> {
        let mut successors = Vec::new();

        //left
        if pos.0 > 0 {
            let new_pos = Pos(pos.0 - 1, pos.1);
            if self.get_cell_by_pos(new_pos) == CellType::Empty ||
                self.get_cell_by_pos(new_pos) == CellType::End
            {
                successors.push(new_pos);
            }
        }

        // right
        if ((pos.0 + 1) as u32) < self.cells_per_row {
            let new_pos = Pos(pos.0 + 1, pos.1);
            if self.get_cell_by_pos(new_pos) == CellType::Empty ||
                self.get_cell_by_pos(new_pos) == CellType::End 
            {
                successors.push(new_pos);
            }
        }

        //up
        if pos.1 > 0 {
            let new_pos = Pos(pos.0, pos.1 - 1);
            if self.get_cell_by_pos(new_pos) == CellType::Empty ||
                self.get_cell_by_pos(new_pos) == CellType::End
            {
                successors.push(new_pos);
            }
        }

        // down
        if ((pos.1 + 1) as u32) < self.cells_per_column {
            let new_pos = Pos(pos.0, pos.1 + 1);
            if self.get_cell_by_pos(new_pos) == CellType::Empty ||
                self.get_cell_by_pos(new_pos) == CellType::End 
            {
                successors.push(new_pos);
            }
        }

        successors
    }

    pub fn is_goal(&self, pos: Pos) -> bool {
        let c = self.get_cell_by_pos(pos);
        if c == CellType::End {
            return true;
        }

        false
    }

    fn find_shortest_path_dijkstra(&self) -> Option<(Vec<Pos>, Vec<Pos>)> {
        let mut searched_cells = Vec::new();

        let shortest_path: Option<_> = dijkstra(
            &self.start_pos,
            |p| { searched_cells.push(p.clone()); self.get_successors(*p).into_iter().map(|p| (p,1_u32)).collect::<Vec<(Pos, u32)>>() },
            |p| self.is_goal(*p)
        );

        shortest_path.map(|sp| (sp.0, searched_cells))
    }

    fn find_shortest_path_astar(&self) -> Option<(Vec<Pos>, Vec<Pos>)> {
        let mut searched_cells = Vec::new();

        let shortest_path: Option<_> = astar(
            &self.start_pos,
            |p| { searched_cells.push(p.clone()); self.get_successors(*p).into_iter().map(|p| (p,1_u32)).collect::<Vec<(Pos, u32)>>() },
            |p| (p.0.abs_diff(self.end_pos.0) + p.1.abs_diff(self.end_pos.1)) as u32,
            |p| self.is_goal(*p)
        );

        shortest_path.map(|sp| (sp.0, searched_cells))
    }

    fn find_shortest_path_fringe(&self) -> Option<(Vec<Pos>, Vec<Pos>)> {
        let mut searched_cells = Vec::new();

        let shortest_path: Option<_> = fringe(
            &self.start_pos,
            |p| { searched_cells.push(p.clone()); self.get_successors(*p).into_iter().map(|p| (p,1_u32)).collect::<Vec<(Pos, u32)>>() },
            |p| (p.0.abs_diff(self.end_pos.0) + p.1.abs_diff(self.end_pos.1)) as u32,
            |p| self.is_goal(*p)
        );

        shortest_path.map(|sp| (sp.0, searched_cells))
    }

    fn find_shortest_path_bfs(&self) -> Option<(Vec<Pos>, Vec<Pos>)> {
        let mut searched_cells = Vec::new();

        let shortest_path: Option<_> = bfs(
            &self.start_pos,
            |p| {searched_cells.push(p.clone()); self.get_successors(*p) },
            |p| self.is_goal(*p)
        );

        shortest_path.map(|sp| (sp, searched_cells))
    }

    fn find_shortest_path_dfs(&self) -> Option<(Vec<Pos>, Vec<Pos>)> {
        let mut searched_cells = HashSet::new();

        let shortest_path: Option<_> = dfs(
            self.start_pos,
            |p| {searched_cells.insert(p.clone()); self.get_successors(*p) },
            |p| self.is_goal(*p)
        );
        shortest_path.map(|sp| (sp, searched_cells.into_iter().collect()))
    }

    pub fn find_shortest_path(&self, algorigthm: Algorithm) -> Option<(Vec<Pos>, Vec<Pos>)> {
        let res = match algorigthm {
            Algorithm::Astar => self.find_shortest_path_astar(),
            Algorithm::Dijkstra => self.find_shortest_path_dijkstra(),
            Algorithm::BreadthFirst => self.find_shortest_path_bfs(),
            Algorithm::Fringe => self.find_shortest_path_fringe(),
        };

        res
    }

}

impl Default for Board {
    fn default() -> Self {
        Board::new(100, 100)
    }
}


