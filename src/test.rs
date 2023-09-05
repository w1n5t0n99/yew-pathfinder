use yew::prelude::*;
use yew_hooks::{use_size, use_window_size, use_renders_count, use_counter};
use yewdux::prelude::*;


#[derive(Default, Clone, PartialEq, Eq, Store)]
struct State {
    count: u32,
}

#[derive(Clone, PartialEq, Eq, Store)]
struct TState {
    count: u32,
    cells: Vec<u32>,
}

impl Default for TState {
    fn default() -> Self {
        Self { count: 0, cells: vec![0,1,2,3], }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Store)]
struct VState {
    vcount: Vec<u32>,
}

#[function_component]
fn ViewCount() -> Html {
    let (state, _) = use_store::<State>();
    let (vstate, _) = use_store::<VState>();

    let count = use_renders_count();

    html! {
        <div class="flex gap-2">
            <p class="flex-initial text-white text-2xl font-body pt-1 mt-2">{"(ViewCount Component)"}</p>
            <p class="bg-gray-400 text-white text-2xl font-body p-1">{count}{"] Count is: "}{ state.count }</p>
            <p class="bg-gray-400 text-white text-2xl font-body p-1">{count}{"] V Len is: "}{ vstate.vcount.len() }</p>
        </div>
    }
}

#[function_component]
fn IncrementCount() -> Html {
    let (_, dispatch) = use_store::<State>();
    let (_, vdispatch) = use_store::<VState>();

    let count = use_renders_count();
    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);
    let vonclick = vdispatch.reduce_mut_callback(|vstate| vstate.vcount.push(1));

    html! {
        <div class="flex gap-2">
            <p class="flex-initial text-white text-2xl font-body pt-1 mt-2">{"(IncrementCount Component)"}</p>
            <button {onclick} class="text-white text-2xl font-body rounded-md bg-gray-500 hover:bg-gray-600 p-1">{count}{"] Count +1"}</button>
            <button onclick={vonclick} class="text-white text-2xl font-body rounded-md bg-gray-500 hover:bg-gray-600 p-1">{count}{"] V +1"}</button>
        </div>
    }
}

#[function_component]
fn VecValue() -> Html {
    let (vstate, vdispatch) = use_store::<VState>();
    let count = use_renders_count();

    let onclick = vdispatch.reduce_mut_callback(|vstate| {
        let i = vstate.vcount.len() as usize;
        if  i > 0 {
            let e = vstate.vcount[i-1];
            vstate.vcount[i-1] = e+1;
        }
        
    });

    let handle_click = onclick.clone();

    html! {
        <>
            <div class="flex gap-2">
                <p class="flex-initial text-white text-2xl font-body pt-1 mt-2">{"(VecValue Component)"}</p>
                <button {onclick} class="text-white text-2xl font-body rounded-md bg-gray-500 hover:bg-gray-600 p-1">{count}{"] V increment last"}</button>
                <p class="bg-gray-400 text-white text-2xl font-body p-1">{count}{"] V = "}{ vstate.vcount.clone() }</p>
                <p class="bg-gray-400 text-white text-2xl font-body p-1">{count}{"] Len is: "}{ vstate.vcount.len() }</p>
            </div>
            <VecValueChild {handle_click} />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct VProps {
    pub handle_click: Callback<MouseEvent>,
}

#[function_component]
fn VecValueChild(props: &VProps) -> Html {
    let count = use_renders_count();
    let onclick = props.handle_click.clone();

    html! {
        <div class="flex gap-2">
            <p class="flex-initial text-white text-2xl font-body pt-1 mt-2">{"(VecValueChild Component)"}</p>
            <button {onclick} class="text-white text-2xl font-body rounded-md bg-gray-500 hover:bg-gray-600 p-1">{count}{"] V Callback"}</button>
        </div>
    }
}

#[function_component]
fn WindowDimensions() -> Html {
    let count = use_renders_count();
    let counter = use_counter(0);
    let (_, dispatch) = use_store::<VState>();
    let window =  use_window_size();
    let window_msg = use_memo(
        |w| format!("window dimensions {:?}", w),
        window,
    );

    let cloned_window = window.clone();
    let cloned_counter = counter.clone();

    use_effect_with_deps(
        move |_| {
            cloned_counter.increase();
            dispatch.set(VState{vcount: vec![(*counter).try_into().unwrap(); 4] });
        },
        cloned_window,
    );

    html! {
        <div class="flex gap-2">
            <p class="flex-initial text-white text-2xl font-body pt-1 mt-2">{"(WindowDimensions Component)"}</p>
            <p class="bg-gray-400 text-white text-2xl font-body p-1">{count}{"] "}</p>
            <p class="flex-initial text-white text-2xl font-body mt-2">{ (*window_msg).clone() }</p>
        </div>
    }
}

//=================================================================
#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell_type: u32,
    pub index: usize,
    pub handle_click: Callback<(u32, usize)>,
}

#[function_component]
fn Cell(props: &CellProps) -> Html {
    let count = use_renders_count();

    let onclick = {
        let cell_type = props.cell_type.clone();
        let index = props.index;
        let hclick = props.handle_click.clone();
        Callback::from(move |e: MouseEvent|{
            hclick.emit((cell_type+1, index));
        })
    };

    html! {
        <>
            if props.cell_type == 0 {
                <td {onclick} class="w-10 border border-slate-600 bg-slate-300">{count}{"]"}</td>
            }
            else if props.cell_type == 1 {
                <td {onclick} class="w-10 border border-slate-600 bg-start">{count}{"]"}</td>
            }
            else if props.cell_type == 2 {
                <td {onclick} class="w-10 border border-slate-600 bg-end">{count}{"]"}</td>
            }
            else {
                <td {onclick} class="w-10 border border-slate-600 bg-orange-400">{count}{"]"}</td>
            }
        </>
    }
}

#[function_component]
fn Board() -> Html {
    let count = use_renders_count();
    let (store, dispatch) = use_store::<TState>();

    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);

    let handle_click = use_callback(move |c: (u32, usize), d| { d.reduce_mut(|state| {state.cells[c.1] = c.0; })}, dispatch.clone());


    html! {
        <div class="flex gap-2">
            <p class="flex-initial text-white text-2xl font-body pt-1 mt-2">{"(Board Component) "}{count}{"] "}</p>
            <button {onclick} class="text-white text-2xl font-body rounded-md bg-gray-500 hover:bg-gray-600 p-1">{"Count +1"}</button>
            <p class="bg-gray-400 text-white text-2xl font-body p-1">{store.count}</p>
            <table class="border-collapse border border-slate-500">
                <thead>
                </thead>
                <tr>
                    <Cell handle_click={handle_click.clone()} cell_type={store.cells[0]} index={0} />
                    <Cell handle_click={handle_click.clone()} cell_type={store.cells[1]} index={1} />
                    <Cell handle_click={handle_click.clone()} cell_type={store.cells[2]} index={2} />
                    <Cell handle_click={handle_click.clone()} cell_type={store.cells[3]} index={3} />
                 </tr>
            </table>
        </div>
    }
}


#[function_component]
pub fn TestApp() -> Html {
    let (_, dispatch) = use_store::<State>();
    let count = use_renders_count();
    let window =  use_window_size();
    let window_msg = use_memo(
        |w| format!("window dimensions {:?}", w),
        window,
    );

    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);

    html! {
        <div class="flex flex-col mx-4 my-4">
            <ViewCount />
            <IncrementCount />
            <VecValue />
            <WindowDimensions />
            <Board />
            <div class="flex gap-2">
                <p class="flex-initial text-white text-2xl font-body pt-1 mt-2">{"(Parent Component)"}</p>
                <button {onclick} class="flex-initial text-white text-2xl font-body rounded-md bg-gray-500 hover:bg-gray-600 p-1 mt-2">{count}{"] Count +1"}</button>
                <p class="flex-initial text-white text-2xl font-body mt-2">{ (*window_msg).clone() }</p>
            </div>
        </div>
    }
}