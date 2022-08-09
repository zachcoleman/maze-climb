use yew::prelude::*;

mod cell;
mod maze;
mod position;

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    html! {
        <>
            < maze::Maze M=10 N=10/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
