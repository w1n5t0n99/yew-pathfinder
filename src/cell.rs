use yew::prelude::*;
use yew_hooks::{use_renders_count, use_window_size, use_timeout, use_unmount, UseTimeoutHandle};
use yewdux::prelude::*;

use crate::board::CellType;


const EMPTY_CLASSES: &str = "w-full h-full hover:bg-slate-700 duration-300";
const WALL_CLASSES: &str = "w-full h-full animate-wall bg-slate-800 hover:bg-slate-500 duration-300";
const START_CLASSES: &str = "w-full h-full bg-start";
const END_CLASSES: &str = "w-full h-full bg-end";
const VISITED_CLASSES: &str = "w-full h-full animate-visited bg-blue-500";
const SHORTEST_PATH_CLASSES: &str = "w-full h-full animate-shortestpath bg-yellow-300";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cell_type: CellType,
    pub index: usize,
    pub on_cell_mousedown: Callback<(CellType, usize)>,
    pub on_cell_enter: Callback<(CellType, usize)>,
}

#[function_component(Cell)]
pub fn cell_component(props: &Props) -> Html {
    //let count = use_renders_count();
    let anim_cell_classes = use_state_eq(||EMPTY_CLASSES.to_string());

    let (cell_classes, delay) = match props.cell_type {
        // when the cell is changed to empty we need to reset animation classes
        CellType::Empty => { anim_cell_classes.set(EMPTY_CLASSES.to_string()); (EMPTY_CLASSES.to_string(), 0) },
        CellType::Wall => { (WALL_CLASSES.to_string(), 0) },
        CellType::Start => { (START_CLASSES.to_string(), 0) },
        CellType::End => { (END_CLASSES.to_string(), 0) },
        CellType::Visited(delay) => { ((*anim_cell_classes).clone(), delay) },
        CellType::ShortestPath(delay) => { ((*anim_cell_classes).clone(), delay) },
    };
    
    let onmouseenter = {
        let cell_type = props.cell_type.clone();
        let index = props.index;
        let hclick = props.on_cell_enter.clone();

        Callback::from(move |_e: MouseEvent|{
            hclick.emit((cell_type.clone(), index));
        })
    };

    let onmousedown = {
        let cell_type = props.cell_type.clone();
        let index = props.index;
        let hclick = props.on_cell_mousedown.clone();

        Callback::from(move |e: MouseEvent|{
            hclick.emit((cell_type.clone(), index));
            e.prevent_default()
        })
    };

    let _timeout = {
        // create timeout to delay applying cell animation
        let cloned_cell_classes = anim_cell_classes.clone();
        let clone_celltype = props.cell_type.clone();

        use_timeout(move || {
            match clone_celltype {
                CellType::Visited(_) => cloned_cell_classes.set(VISITED_CLASSES.to_string()),
                CellType::ShortestPath(_) => cloned_cell_classes.set(SHORTEST_PATH_CLASSES.to_string()),
                _ => {},
            }
        },
        delay as u32)
    };

    
    html! {
        <td {onmouseenter} {onmousedown}  class="select-none border border-gray-300 w-7 h-7">
            <div class={classes!(cell_classes.clone())}></div>
        </td>
    }
}