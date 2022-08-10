use yew::prelude::*;

mod cell;
mod maze;
mod position;

pub enum Msg {
    ClickedCell { pos: (usize, usize) },
    Reset,
}

pub struct App {
    maze: maze::Maze,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut maze = maze::Maze::new(10, 10);
        maze.cut_up_maze(10);
        Self { maze: maze }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickedCell { pos: (a, b) } => {
                self.maze.cells[a * self.maze.n + b].clicked =
                    !self.maze.cells[a * self.maze.n + b].clicked;
                true
            }
            Msg::Reset => {
                self.maze.reset_maze();
                self.maze.cut_up_maze(10);
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
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
