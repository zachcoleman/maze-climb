use rand::{thread_rng, Rng};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Wall {
    No,
    Yes,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    Left: Wall,
    Right: Wall,
    Up: Wall,
    Down: Wall,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    r: usize,
    c: usize,
}

#[wasm_bindgen]
impl Position {
    pub fn apply_move(self, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Up => match self.r.checked_sub(1) {
                Some(r) => Some(Self { r: r, c: self.c }),
                None => None,
            },
            Direction::Down => match self.r.checked_add(1) {
                Some(r) => Some(Self { r: r, c: self.c }),
                None => None,
            },
            Direction::Left => match self.c.checked_sub(1) {
                Some(c) => Some(Self { r: self.r, c: c }),
                None => None,
            },
            Direction::Right => match self.c.checked_add(1) {
                Some(c) => Some(Self { r: self.r, c: c }),
                None => None,
            },
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Maze {
    M: usize,
    N: usize,
    grid: Vec<Cell>,
}

#[wasm_bindgen]
impl Maze {
    pub fn new(M: usize, N: usize) -> Self {
        Maze {
            M: M,
            N: N,
            grid: (0..M * N)
                .map(|_| Cell {
                    Left: Wall::Yes,
                    Right: Wall::Yes,
                    Up: Wall::Yes,
                    Down: Wall::Yes,
                })
                .collect(),
        }
    }

    // pub fn get_cell(&mut self, p: Position) -> &mut Cell{
    //     &mut self.grid[p.r*self.N + p.c]
    // }

    pub fn cut_up_maze(&mut self) {
        // since our closure mutates self we must get all
        // our constants and information
        // before the mutable borrow occurs
        let num_cells = self.M * self.N;
        let (M, N) = (self.M, self.N);
        let mut pos = Position { r: 0, c: 0 };
        let mut visited: HashSet<Position> = HashSet::new();

        let mut step = |p: Position,
                        visited: &mut HashSet<Position>,
                        depth_count: usize|
         -> Option<Position> {
            // add pos to visited
            visited.insert(p);

            // borrow and find valid neighbors
            let mut neighbor_dirs = vec![];
            for dir in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some(new_pos) = p.apply_move(dir) {
                    // if starting a new section, make sure to connect something visited
                    if depth_count == 0 {
                        if self.is_position_valid(new_pos) && visited.contains(&new_pos) {
                            neighbor_dirs.push(dir);
                        }
                    } else {
                        if self.is_position_valid(new_pos) && !visited.contains(&new_pos) {
                            neighbor_dirs.push(dir);
                        }
                    }
                }
            }
            if neighbor_dirs.len() == 0 {
                return None;
            }

            // get mutable reference to cell
            // and get random direction to neighbor
            let mut cell = self.grid[p.r * self.N + p.c];
            let dir = neighbor_dirs[thread_rng().gen_range(0..neighbor_dirs.len())];
            let new_pos = p.apply_move(dir).unwrap();

            match dir {
                Direction::Left => {
                    cell.Left = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Right = Wall::No;
                }
                Direction::Right => {
                    cell.Right = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Left = Wall::No;
                }
                Direction::Down => {
                    cell.Down = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Up = Wall::No;
                }
                Direction::Up => {
                    cell.Up = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Down = Wall::No;
                }
            }
            Some(new_pos)
        };

        let max_depth = M * N / 3;
        let mut depth_count = 0;
        while visited.len() < num_cells {
            match (
                depth_count < max_depth,
                step(pos, &mut visited, depth_count),
            ) {
                (true, Some(p)) => {
                    pos = p;
                    depth_count += 1;
                }
                _ => {
                    depth_count = 0;
                    'outer: for i in 0..M {
                        for j in 0..N {
                            let p = Position { r: i, c: j };
                            if !visited.contains(&p) {
                                pos = p;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn is_position_valid(&self, p: Position) -> bool {
        p.r < self.M && p.c < self.N
    }

    pub fn pretty_render(&self) -> String {
        let mut s: String = String::from("");
        for row_num in 0..self.M {
            let mut top_row = String::from(" ");
            // let mut mid_row = String::from("");
            let mut mid_row = String::from("|");
            let mut bot_row = String::from(" ");
            for j in 0..self.N {
                let cell = self.grid[row_num * self.N + j];
                match cell.Up {
                    Wall::No => {
                        top_row += "   ";
                    }
                    Wall::Yes => {
                        top_row += " _ ";
                    }
                }
                match (cell.Left, cell.Right) {
                    (_, Wall::No) => {
                        mid_row += "   ";
                    }
                    (_, Wall::Yes) => {
                        mid_row += "  |";
                    } // (Wall::No, Wall::No) => { mid_row += "   "; }
                      // (Wall::No, Wall::Yes) => { mid_row += "  |"; }
                      // (Wall::Yes, Wall::No) => { mid_row += "|  "; }
                      // (Wall::Yes, Wall::Yes) => { mid_row += "| |"; }
                }
                match cell.Down {
                    Wall::No => {
                        bot_row += "   ";
                    }
                    Wall::Yes => {
                        bot_row += " ‾ ";
                    }
                }
            }
            s += &(top_row + "\n");
            s += &(mid_row + "\n");
            // s += &(bot_row + "\n");
        }
        // add nice bottom
        s += &" ";
        s += &(" _ ".repeat(self.N));
        s
    }
}

#[function_component(App)]
fn app() -> Html {
    let mut maze = Maze::new(6, 6);
    let before = maze.pretty_render().to_string();
    maze.cut_up_maze();
    let after = maze.pretty_render().to_string();

    html! {
        <>
            <svg width="1600" height="1600">
                <rect x="0" y="0" width="10" height="80"
                style="fill:black;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
                <rect x="70" y="0" width="10" height="80"
                style="fill:black;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
                <rect x="0" y="0" width="80" height="10"
                style="fill:black;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
                <rect x="0" y="70" width="80" height="10"
                style="fill:black;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
            </svg>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
