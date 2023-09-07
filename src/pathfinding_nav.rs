use gloo::console::info;
use yew::prelude::*;
use yew_hooks::{use_window_size, use_renders_count, use_event_with_window, use_size};
use yewdux::prelude::*;
use pathfinding::prelude::dijkstra;

use crate::{nav_state::NavState, board::{Board, Pos}};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub board_dispatch: Dispatch<Board>,
}



#[function_component]
pub fn PathfindingNav(props: &Props) -> Html {
    //let count = use_renders_count();
    let (nav_store, nav_dispatch) = use_store::<NavState>();

    let on_visclick = {
        let cloned_nav_dispatch = nav_dispatch.clone();
        let cloned_board_dispatch = props.board_dispatch.clone();
        Callback::from(move |_e: MouseEvent|{
            let board = cloned_board_dispatch.get();
            let nav_state = cloned_nav_dispatch.get();
            let result = board.find_shortest_path(nav_state.algorithm.clone());

            if let Some((shortest_path, visited)) = result {
                cloned_board_dispatch.reduce_mut(move |state| {
                    state.set_cell_by_pathfinding(&visited, &shortest_path, 6);
                })
            }
        })
    };

    let on_cbclick = {
        let cloned_board_dispatch = props.board_dispatch.clone();
        Callback::from(move |_e: MouseEvent|{
            cloned_board_dispatch.reduce_mut(|state| state.clear())
        })
    };

    html! {
        <>
            //<p class="z-2 absolute bg-nav rounded-lg text-white text-2xl font-body p-1 ml-1 mt-28">{count}{"] Nav"}</p> 
            <div class="flex flex-wrap justify-center items-center mt-1 mx-auto w-full">
                <div class="bg-nav -skew-x-12 py-1 flex items-center shadow-lg">
                    <h1 class="text-xl text-white font-body font-medium mx-2">{"Pathfinding Visualizer"}</h1>
                    <div class="divide-x inline-block p-2">
                        <div class="inline-block">
                            <button onclick={on_visclick} class="text-base text-white bg-green-500 hover:bg-green-400 font-body rounded-md p-1 mx-2">{"Visualize!"}</button>
                            <button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Algorithms: Dijkstra's Algorithm"}</button>
                            <button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Speed: Fast"}</button>
                        </div>
                        <div class="inline-block">
                            <button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Mazes: None"}</button>
                            <button onclick={on_cbclick} class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Clear Board"}</button>
                            <button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Clear Path"}</button>
                        </div> 
                    </div>
                </div>
            </div>
        </>
    }
}