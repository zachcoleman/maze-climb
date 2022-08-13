use std::collections::HashSet;
use yew::prelude::*;

mod cell;
mod maze;
mod position;

use position::Position;

pub enum Msg {
    ClickedCell { pos: (usize, usize) },
    Reset,
}

pub struct App {
    maze: maze::Maze,
    path: HashSet<Position>,
    solved: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut maze = maze::Maze::new(4, 4);
        let path: HashSet<Position> = HashSet::new();
        maze.cut_up_maze(10);
        Self {
            maze: maze,
            path: path,
            solved: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickedCell { pos: (a, b) } => {
                // flip a cell
                self.maze.cells[a * self.maze.n + b].clicked =
                    !self.maze.cells[a * self.maze.n + b].clicked;

                // if flipped to clicked add to path
                if self.maze.cells[a * self.maze.n + b].clicked {
                    self.path.insert(Position { r: a, c: b });
                } else {
                    self.path.remove(&Position { r: a, c: b });
                }

                // check current path and see if complete
                if self.maze.is_connected(
                    position::Position { r: 0, c: 0 },
                    position::Position {
                        r: self.maze.m - 1,
                        c: self.maze.n - 1,
                    },
                    &self.path,
                ) {
                    self.solved = true;
                } else {
                    self.solved = false;
                }
                true
            }
            Msg::Reset => {
                self.maze.reset_maze();
                self.maze.cut_up_maze(10);
                self.path = HashSet::new();
                self.solved = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // define callbacks
        let reset_maze = ctx.link().callback(|_| Msg::Reset);
        let clicked_cell = ctx
            .link()
            .callback(|(a, b)| Msg::ClickedCell { pos: (a, b) });

        html! {
            <>
                <maze::MazeView
                    maze={ self.maze.clone() }
                    click_callback={ clicked_cell }
                    reset_callback={ reset_maze }
                />
                if self.solved{
                    <p> { "Solved!" } </p>
                }
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
