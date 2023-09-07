use yew::prelude::*;
use yew_hooks::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub items: Vec<AttrValue>,
    #[prop_or_default]
    pub on_selected: Callback<AttrValue>,
}

#[function_component]
pub fn Dropdown(props: &Props) -> Html {
    let default_item = props.items.first().map(|s| s.clone()).unwrap_or(AttrValue::Static(""));

    let show_dropdown = use_state(|| false);
    let selected_item = use_state(|| default_item);

    let onclick = {
        let cloned_show_dropdown = show_dropdown.clone();

        Callback::from(move |_: MouseEvent| {
            cloned_show_dropdown.set(true);
        })
    };

    let onfocusout = {
        let cloned_show_dropdown = show_dropdown.clone();

        Callback::from(move |_: FocusEvent| {
            cloned_show_dropdown.set(false);
        })
    };

    html!{
        <>
            <button {onclick} class="text-base text-white hover:text-green-500 bg-transparent font-body mx-2">{(*selected_item).as_str()}</button>
            if *show_dropdown {
                <div {onfocusout} class="bg-nav absolute mt-2 py-2 px-2 -skew-x-2 shadow-xl z-10">
                    <div class="flex flex-col items-center gap-1 overflow-auto">
                    {
                        props.items.clone().iter().map(|item| {
                            html!{<div class="w-full cursor-pointer text-white hover:text-green-500 bg-transparent font-body">{ item.clone() }</div>}
                        }).collect::<Html>()
                    }
                    </div>
                </div>
            }
        </>
    }
}