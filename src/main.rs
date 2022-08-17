use gloo_timers::callback::Interval;
use std::collections::HashSet;
use yew::prelude::*;

mod cell;
mod game;
mod maze;
mod position;

use position::Position;

pub enum Msg {
    ClickedCell { pos: (usize, usize) },
    NewGame,
    Tick,
    Reset,
}

pub struct App {
    game: game::Game,
    maze: maze::Maze,
    path: HashSet<Position>,
    lost: bool,
    timer: usize,
    _interval: Interval,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let game = game::Game::new();
        let maze = game.get_maze();

        let path: HashSet<Position> = HashSet::new();

        let timer_callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(1_000, move || timer_callback.emit(()));

        Self {
            game: game,
            maze: maze,
            path: path,
            lost: false,
            timer: 15,
            _interval: interval,
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
                    self.game.apply_win();
                    self.maze = self.game.get_maze();
                    self.path = HashSet::new();
                    self.timer = 15;
                }
                true
            }
            Msg::Reset => {
                if self.game.lives > 1 {
                    self.game.apply_loss();
                    self.maze = self.game.get_maze();
                    self.path = HashSet::new();
                    true
                } else {
                    false
                }
            }
            Msg::NewGame => {
                self.game = game::Game::new();
                self.maze = self.game.get_maze();
                self.path = HashSet::new();
                self.lost = false;
                self.timer = 15;
                true
            }
            Msg::Tick => {
                if self.timer > 0 {
                    self.timer -= 1;
                    true
                } else {
                    match self.game.apply_loss() {
                        game::GameStatus::Alive => {
                            self.timer = 15;
                            true
                        }
                        game::GameStatus::Dead => {
                            self.lost = true;
                            true
                        }
                    }
                }
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
                <button onclick={ctx.link().callback(|_| Msg::NewGame)}>{ "New Game" }</button>
                if self.lost{
                    <p> { "You lost!" } </p>
                } else{
                    <p> { "Level: " } { self.game.level } </p>
                    <p> { "Lives: " } { self.game.lives } </p>
                    <p> { "Time: " } { self.timer } </p>
                    <maze::MazeView
                        maze={ self.maze.clone() }
                        click_callback={ clicked_cell }
                        reset_callback={ reset_maze }
                    />
                }
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
