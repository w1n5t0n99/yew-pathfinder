use yew::prelude::*;
use yew_hooks::{use_renders_count, use_window_size, use_timeout, use_unmount};
use yewdux::prelude::*;

use crate::board::CellType;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub cell_type: CellType,
    pub index: usize,
    pub on_cell_mousedown: Callback<(CellType, usize)>,
    pub on_cell_enter: Callback<(CellType, usize)>,
}

#[function_component(Cell)]
pub fn cell_component(props: &Props) -> Html {
   let count = use_renders_count();
   let cell_classes = use_state(||"w-full h-full hover:bg-slate-700 duration-300".to_string());
    
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

    let timeout = {
        // create timeout to delay applying cell animation
        let cloned_cell_classes = cell_classes.clone();
        let clone_celltype = props.cell_type.clone();

        let delay = match clone_celltype {
            CellType::Visited(delay) => delay,
            CellType::ShortestPath(delay) => delay,
            _ => 0,
        };

        use_timeout(move || {
            match clone_celltype {
                CellType::Visited(_) => cloned_cell_classes.set("w-full h-full animate-visited bg-blue-500".to_string()),
                CellType::ShortestPath(_) => cloned_cell_classes.set("w-full h-full animate-shortestpath bg-yellow-300".to_string()),
                _ => {},
            }
        },
        delay as u32)
    };

    use_unmount(move || {
        timeout.cancel();
    });
    
    html! {
        <td {onmouseenter} {onmousedown}  class="select-none border border-gray-300 w-7 h-7">
            if let CellType::Visited(_) = props.cell_type {
                <div class={classes!((*cell_classes).clone())}></div>
                //<div class="w-full h-full animate-visited bg-blue-400"></div>
            }
            else if let CellType::ShortestPath(_) = props.cell_type {
                <div class={classes!((*cell_classes).clone())}></div>
                //<div class="w-full h-full animate-shortestpath bg-yellow-200"></div>
            }
            else if props.cell_type == CellType::End {
                <div class="w-full h-full bg-end"></div>
            }
            else if props.cell_type == CellType::Start {
                <div class="w-full h-full bg-start"></div>
            }
            else if props.cell_type == CellType::Wall{
                <div class="w-full h-full animate-wall bg-slate-800 hover:bg-slate-500 duration-300"></div>
            }
            else {
                <div class="w-full h-full hover:bg-slate-700 duration-300"></div>
            }
        </td>
    }
}