use gloo::console::info;
use yew::prelude::*;
use yew_hooks::{use_window_size, use_renders_count, use_event_with_window, use_size};
use yewdux::prelude::*;

use crate::{board::{Board, CellType}, cell::Cell, nav_state::NavState, pathfinding_nav::PathfindingNav};


#[function_component]
pub fn PathfindingVisualizer() -> Html {
    let count = use_renders_count();
    //let window = use_window_size();
    let (board_store, board_dispatch) = use_store::<Board>();
    let (_, nav_dispatch) = use_store::<NavState>();

    let board_node =  use_node_ref();
    let board_dimensions = use_size(board_node.clone());

    {
        let cloned_board_dispatch = board_dispatch.clone();
        let cloned_nav_dispatch = nav_dispatch.clone();
        use_effect_with_deps(
            move |w| {
                cloned_board_dispatch.reduce_mut(|state| {
                    state.reset(w.0 as u32, w.1 as u32);
                });

                cloned_nav_dispatch.reduce_mut(|state| {
                    state.reset();
                });
            },
            board_dimensions,
        );
    }

    {
        let cloned_board_dispatch = board_dispatch.clone();
        use_event_with_window("mouseup", move |_e: MouseEvent| {
            cloned_board_dispatch.reduce_mut(|state| {state.allow_drawing = false; })
        });
    }

    let on_cell_enter = use_callback(move |c: (CellType, usize), dispatch,| {
            let store = dispatch.get();
            if store.allow_drawing == true {
                if c.0 == CellType::Empty {
                     dispatch.reduce_mut(|state| {state.set_wall_by_index(c.1); });
                }
                else if c.0 == CellType::Wall  {
                    dispatch.reduce_mut(|state| {state.clear_cell_by_index(c.1); });
                }
            }
        },
        board_dispatch.clone()
    );

    let on_cell_mousedown = use_callback(move |c: (CellType, usize), dispatch,| {
            if c.0 == CellType::Empty {
                dispatch.reduce_mut(|state| {state.set_wall_by_index(c.1); state.allow_drawing = true; });
            }
            else if c.0 == CellType::Wall {
                dispatch.reduce_mut(|state| {state.clear_cell_by_index(c.1); state.allow_drawing = true; });
            }

        },
        board_dispatch.clone()
    );  
    

    html! {
        <div class="h-screen flex flex-col gap-2">
            //<p class="z-2 absolute bg-nav rounded-lg text-white text-2xl font-body p-1 ml-1 mt-16">{count}{"] Board: "}{board_store.cells.len()}</p> 
            //<p class="z-1 absolute bg-nav rounded-lg text-white text-2xl font-body p-1 ml-1 mt-28">{"Window: "}{window.0}{"-"}{window.1}</p> 
            //<p class="z-1 absolute bg-nav rounded-lg text-white text-2xl font-body p-1 ml-1 mt-52">{"Allow Drawing: "}{board_store.allow_drawing}</p> 
            <PathfindingNav board_dispatch={board_dispatch.clone()} />            
            <div class="grow min-h-0" ref={board_node}>
                <table class="border-collapse border border-gray-700 mx-auto">
                    <thead>
                    </thead>
                    <tbody>
                    
                    {
                        board_store.cells.chunks(board_store.cells_per_row as usize).enumerate().map(|(col_num, row)| {
                            html!{
                                <tr>
                                {
                                    row.iter()
                                        .enumerate()
                                        .map(|(row_rum, cell)| { html!{ <Cell on_cell_enter={on_cell_enter.clone()} on_cell_mousedown={on_cell_mousedown.clone()} cell_type={cell.clone()} index={(col_num * (board_store.cells_per_row as usize)) + row_rum} /> }})
                                        .collect::<Html>()
                                }
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    
                    </tbody>
                </table>
            </div>            
        </div>
    }
}

