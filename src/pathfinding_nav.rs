use std::vec;

use gloo::console::info;
use yew::prelude::*;
use yew_hooks::{use_window_size, use_renders_count, use_event_with_window, use_size};
use yewdux::prelude::*;
use pathfinding::prelude::dijkstra;

use crate::{nav_state::{NavState, Speed}, board::{Board, Pos, Algorithm}, dropdown::Dropdown};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub board_dispatch: Dispatch<Board>,
}



#[function_component]
pub fn PathfindingNav(props: &Props) -> Html {
    //let count = use_renders_count();
    let (nav_store, nav_dispatch) = use_store::<NavState>();

    let on_visualize_click = {
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

    let on_clear_board_click = {
        let cloned_board_dispatch = props.board_dispatch.clone();
        Callback::from(move |_e: MouseEvent|{
            cloned_board_dispatch.reduce_mut(|state| state.clear())
        })
    };

    let algo_items = vec![AttrValue::from("Astar"), AttrValue::from("Dijkstra"), AttrValue::from("BreadthFirst"), AttrValue::from("DepthFirst")];
    let speed_items = vec![AttrValue::from("Fast"), AttrValue::from("Average"), AttrValue::from("Slow")];

    let on_algo_selected = {
        let cloned_nav_dispatch = nav_dispatch.clone();
        Callback::from(move |selected: AttrValue| {
            cloned_nav_dispatch.reduce_mut(|state| {
                match selected.as_str() {
                    "Astar" => state.algorithm = Algorithm::Astar,
                    "Dijkstra" => state.algorithm = Algorithm::Dijkstra,
                    "BreadthFirst" => state.algorithm = Algorithm::BreadthFirst,
                    "DepthFirst" => state.algorithm = Algorithm::DepthFirst,
                    _ => panic!("nonvalid algorithm")
                }
            })
        })
    };

    let on_speed_selected = {
        let cloned_nav_dispatch = nav_dispatch.clone();
        Callback::from(move |selected: AttrValue| {
            cloned_nav_dispatch.reduce_mut(|state| {
                match selected.as_str() {
                    "Fast" => state.speed = Speed::Fast,
                    "Average" => state.speed = Speed::Average,
                    "Slow" => state.speed = Speed::Slow,
                    _ => panic!("nonvalid speed")
                }
            })
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
                            <button onclick={on_visualize_click} class="text-base text-white bg-green-500 hover:bg-green-400 font-body rounded-md p-1 mx-2">{"Visualize!"}</button>
                           // <button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Algorithms: Dijkstra's Algorithm \u{23F7}"}</button>
                            <Dropdown on_selected={on_algo_selected} items={algo_items} />
                            <Dropdown on_selected={on_speed_selected} items={speed_items} />
                            //<button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Speed: Fast \u{23F7}"}</button>
                        </div>
                        <div class="inline-block">
                            <button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Mazes: None \u{23F7}"}</button>
                            <button onclick={on_clear_board_click} class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Clear Board"}</button>
                            <button class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{"Clear Path"}</button>
                        </div> 
                    </div>
                </div>
            </div>
        </>
    }
}