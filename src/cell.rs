use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Wall {
    No,
    Yes,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub Left: Wall,
    pub Right: Wall,
    pub Up: Wall,
    pub Down: Wall,
}

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell: Cell,
}

/// TODO: use a CSS component to style cells same
#[function_component(CellComponent)]
pub fn html_cell(CellProps { cell }: &CellProps) -> Html {
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
