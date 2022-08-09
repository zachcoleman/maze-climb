use rand::{thread_rng, Rng};
use std::collections::HashSet;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Wall {
    No,
    Yes,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    Left: Wall,
    Right: Wall,
    Up: Wall,
    Down: Wall,
}

#[derive(Properties, PartialEq)]
pub struct CellProps {
    cell: Cell,
}

#[function_component(CellComponent)]
fn html_cell(CellProps { cell }: &CellProps) -> Html {
    html! {
        <svg width="80" height="80">
            if cell.Left == Wall::Yes{
                <rect x="0" y="0" width="10" height="80"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
            } else {
                <rect x="0" y="0" width="10" height="80"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
            }
            if cell.Right == Wall::Yes{
                <rect x="70" y="0" width="10" height="80"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
            } else{
                <rect x="70" y="0" width="10" height="80"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
            }
            if cell.Up == Wall::Yes{
                <rect x="0" y="0" width="80" height="10"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
            }else{
                <rect x="0" y="0" width="80" height="10"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
            }
            if cell.Down == Wall::Yes{
                <rect x="0" y="70" width="80" height="10"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
            }else{
                <rect x="0" y="70" width="80" height="10"
                style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
            }
        </svg>
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    r: usize,
    c: usize,
}

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

#[derive(Clone, Debug)]
pub struct Maze {
    M: usize,
    N: usize,
    grid: Vec<Cell>,
}

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

    pub fn reset_maze(&mut self) {
        *self = Self::new(self.M, self.N);
    }

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
                    self.grid[p.r * self.N + p.c].Left = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Right = Wall::No;
                }
                Direction::Right => {
                    self.grid[p.r * self.N + p.c].Right = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Left = Wall::No;
                }
                Direction::Down => {
                    self.grid[p.r * self.N + p.c].Down = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Up = Wall::No;
                }
                Direction::Up => {
                    self.grid[p.r * self.N + p.c].Up = Wall::No;
                    self.grid[new_pos.r * self.N + new_pos.c].Down = Wall::No;
                }
            }
            log::info!("{:?}, {:?}", p, self.grid[p.r * N + p.c]);
            log::info!("{:?}, {:?}", new_pos, self.grid[new_pos.r * N + new_pos.c]);
            Some(new_pos)
        };

        let max_depth = usize::max((M * N) / 2, 4);
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

    pub fn console_render(&self) -> String {
        let mut s: String = String::from("\n");
        for row_num in 0..self.M {
            let mut top_row = String::from(" ");
            let mut mid_row = String::from("|");
            for j in 0..self.N {
                let cell = self.grid[row_num * self.N + j];
                match cell.Up {
                    Wall::No => {
                        top_row += "...";
                    }
                    Wall::Yes => {
                        top_row += "._.";
                    }
                }
                match (cell.Left, cell.Right) {
                    (_, Wall::No) => {
                        mid_row += "...";
                    }
                    (_, Wall::Yes) => {
                        mid_row += "..|";
                    }
                }
            }
            s += &(top_row + "\n");
            s += &(mid_row + "\n");
        }
        s += &" ";
        s += &(" _ ".repeat(self.N));
        s
    }
}

pub enum Msg {
    Reset,
}

impl Component for Maze {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut maze = Maze::new(10, 10);
        maze.cut_up_maze();
        maze
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.reset_maze();
                self.cut_up_maze();
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        // get maze rows
        let mut maze_rows = vec![];
        for row_num in 0..self.M {
            maze_rows.push(html! {
                <p style="margin:0; padding:0;" >
                    {
                        self.grid[row_num*self.N..(row_num+1)*self.N].iter().map(|cell| html!{
                            < CellComponent cell={ cell.clone() } />
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

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let mut maze = Maze::new(14, 20);
    maze.cut_up_maze();
    log::info!("{}", maze.console_render().to_string());

    let mut maze_rows = vec![];
    for row_num in 0..maze.M {
        maze_rows.push(html! {
            <p style="margin:0; padding:0;" >
                {
                    maze.grid[row_num*maze.N..(row_num+1)*maze.N].iter().map(|cell| html!{
                        < CellComponent cell={ cell.clone() } />
                    }).collect::<Html>()
                }
            </p>
        });
    }
    html! {
        <>
            { maze_rows }
        </>
    }
}

fn main() {
    yew::start_app::<Maze>();
    // yew::start_app::<App>();
}
