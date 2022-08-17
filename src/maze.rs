use rand::{thread_rng, Rng};
use std::collections::HashSet;
use yew::prelude::*;
use yew::Callback;

use crate::cell::{Cell, CellView, Wall};
use crate::position::{Direction, Position};

#[derive(Clone, Debug, PartialEq)]
pub struct Maze {
    pub m: usize,
    pub n: usize,
    pub cells: Vec<Cell>,
}

impl Maze {
    /// constructor for a completely closed-off Maze
    pub fn new(m: usize, n: usize) -> Self {
        Maze {
            m: m,
            n: n,
            cells: (0..m * n)
                .map(|_| Cell {
                    left: Wall::Yes,
                    right: Wall::Yes,
                    up: Wall::Yes,
                    down: Wall::Yes,
                    clicked: false,
                })
                .collect(),
        }
    }

    /// make a Maze like a new closed-off Maze
    #[allow(dead_code)]
    pub fn reset_maze(&mut self) {
        *self = Self::new(self.m, self.n);
    }

    /// randomize the Maze with paths
    pub fn cut_up_maze(&mut self, max_depth: usize) {
        // since our closure mutates self we must get all
        // our constants and information in variables prior
        // to the mutable borrow in closure
        let num_cells = self.m * self.n;
        let (m, n) = (self.m, self.n);
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
            let dir = neighbor_dirs[thread_rng().gen_range(0..neighbor_dirs.len())];
            let new_pos = p.apply_move(dir).unwrap();
            match dir {
                Direction::Left => {
                    self.cells[p.r * self.n + p.c].left = Wall::No;
                    self.cells[new_pos.r * self.n + new_pos.c].right = Wall::No;
                }
                Direction::Right => {
                    self.cells[p.r * self.n + p.c].right = Wall::No;
                    self.cells[new_pos.r * self.n + new_pos.c].left = Wall::No;
                }
                Direction::Down => {
                    self.cells[p.r * self.n + p.c].down = Wall::No;
                    self.cells[new_pos.r * self.n + new_pos.c].up = Wall::No;
                }
                Direction::Up => {
                    self.cells[p.r * self.n + p.c].up = Wall::No;
                    self.cells[new_pos.r * self.n + new_pos.c].down = Wall::No;
                }
            }
            Some(new_pos)
        };

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
                    'outer: for i in 0..m {
                        for j in 0..n {
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

    pub fn is_connected(&self, start: Position, stop: Position, path: &HashSet<Position>) -> bool {
        // let's do BFS starting from our start position
        let mut queue: Vec<Position> = vec![start];
        let mut visited: HashSet<Position> = HashSet::new();

        while queue.len() > 0 {
            let mut tmp = vec![];
            for p in &queue {
                for dir in vec![
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    if let Some(new_pos) = p.apply_move(dir) {
                        // check if new_pos in path
                        if path.contains(&new_pos) && !visited.contains(&new_pos) {
                            // check correct walls are missing
                            match dir {
                                Direction::Left => {
                                    // check left wall of pos and check right wall of new_pos
                                    if self.cells[p.r * self.n + p.c].left == Wall::No
                                        && self.cells[new_pos.r * self.n + new_pos.c].right
                                            == Wall::No
                                    {
                                        if new_pos == stop {
                                            return true;
                                        }
                                        tmp.push(new_pos);
                                    }
                                }
                                Direction::Right => {
                                    if self.cells[p.r * self.n + p.c].right == Wall::No
                                        && self.cells[new_pos.r * self.n + new_pos.c].left
                                            == Wall::No
                                    {
                                        if new_pos == stop {
                                            return true;
                                        }
                                        tmp.push(new_pos);
                                    }
                                }
                                Direction::Up => {
                                    if self.cells[p.r * self.n + p.c].up == Wall::No
                                        && self.cells[new_pos.r * self.n + new_pos.c].down
                                            == Wall::No
                                    {
                                        if new_pos == stop {
                                            return true;
                                        }
                                        tmp.push(new_pos);
                                    }
                                }
                                Direction::Down => {
                                    if self.cells[p.r * self.n + p.c].down == Wall::No
                                        && self.cells[new_pos.r * self.n + new_pos.c].up == Wall::No
                                    {
                                        if new_pos == stop {
                                            return true;
                                        }
                                        tmp.push(new_pos);
                                    }
                                }
                            }
                        }
                        visited.insert(p.clone());
                    }
                }
            }
            queue = tmp;
        }
        false
    }

    pub fn is_position_valid(&self, p: Position) -> bool {
        p.r < self.m && p.c < self.n
    }
}

pub enum Msg {
    Reset,
}

pub struct MazeView;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MazeViewProps {
    pub maze: Maze,
    pub click_callback: Callback<(usize, usize)>,
    pub reset_callback: Callback<()>,
}

impl Component for MazeView {
    type Message = Msg;
    type Properties = MazeViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                ctx.props().reset_callback.emit(());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut maze_rows = vec![];
        for row_num in 0..ctx.props().maze.m {
            maze_rows.push(html! {
                <p style="margin:0; padding:0;" >
                    {
                        ctx.props().maze.cells[row_num*ctx.props().maze.n..(row_num+1)*ctx.props().maze.n]
                            .iter()
                            .enumerate()
                            .map(|(j, cell)| html!{
                                < CellView
                                    cell={ cell.clone() }
                                    pos={ (row_num, j) }
                                    cell_clicked={ ctx.props().click_callback.clone() }
                                />
                        }).collect::<Html>()
                    }
                </p>
            });
        }
        html! {
            <div>
                { maze_rows }
                <button onclick={link.callback(|_| Msg::Reset)}>{ "Reset Maze" }</button>
            </div>
        }
    }
}
