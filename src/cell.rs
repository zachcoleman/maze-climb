use yew::prelude::*;
use yew::Callback;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Wall {
    No,
    Yes,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub left: Wall,
    pub right: Wall,
    pub up: Wall,
    pub down: Wall,
    pub clicked: bool,
}

pub enum Msg {
    Click,
}

pub struct CellView;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CellViewProps {
    pub cell: Cell,
    pub pos: (usize, usize),
    pub cell_clicked: Callback<(usize, usize)>,
}

impl Component for CellView {
    type Message = Msg;
    type Properties = CellViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                ctx.props().cell_clicked.emit(ctx.props().pos);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <svg width="80" height="80">
                // draw cell
                if ctx.props().cell.clicked{
                    <rect x="0" y="0" width="80" height="80"
                    onclick={link.callback(|_| Msg::Click)}
                    style="fill:green;fill-opacity:0.3;"/>
                } else{
                    <rect x="0" y="0" width="80" height="80"
                    onclick={link.callback(|_| Msg::Click)}
                    style="fill:grey;fill-opacity:0.3;"/>
                }

                // draw walls
                if ctx.props().cell.left == Wall::Yes{
                    <rect x="0" y="0" width="10" height="80"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
                } else {
                    <rect x="0" y="0" width="10" height="80"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
                }
                if ctx.props().cell.right == Wall::Yes{
                    <rect x="70" y="0" width="10" height="80"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
                } else{
                    <rect x="70" y="0" width="10" height="80"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
                }
                if ctx.props().cell.up == Wall::Yes{
                    <rect x="0" y="0" width="80" height="10"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
                }else{
                    <rect x="0" y="0" width="80" height="10"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
                }
                if ctx.props().cell.down == Wall::Yes{
                    <rect x="0" y="70" width="80" height="10"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.9;stroke-opacity:0.9" />
                }else{
                    <rect x="0" y="70" width="80" height="10"
                    style="fill:blue;stroke:black;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.1" />
                }
            </svg>
        }
    }
}
