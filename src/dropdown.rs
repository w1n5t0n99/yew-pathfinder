use gloo::console::info;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
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
    let count = use_renders_count();
    let default_item = props.items.first().map(|s| s.clone()).unwrap_or(AttrValue::Static(""));

    let show_dropdown = use_state(|| false);
    let selected_item = use_state(|| default_item);

    let onclick = {
        let cloned_show_dropdown = show_dropdown.clone();

        Callback::from(move |_: MouseEvent| {
            if *cloned_show_dropdown == true {
                cloned_show_dropdown.set(false);
            }
            else {
                cloned_show_dropdown.set(true);
            }
            
        })
    };

    let onblur = {
        let cloned_show_dropdown = show_dropdown.clone();

        Callback::from(move |_: FocusEvent| {
            cloned_show_dropdown.set(false);
        })
    };

    let on_item_click = {
        let cloned_on_selected = props.on_selected.clone();
        let cloned_show_dropdown = show_dropdown.clone();
        let cloned_selected_item = selected_item.clone();

        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap();

            if let Some(value) = target.unchecked_into::<HtmlElement>().text_content() {
                cloned_on_selected.emit(value.clone().into());
                cloned_selected_item.set(value.clone().into());
            }

            cloned_show_dropdown.set(false);
        })
    };

    html!{
        <>
        <div class="inline-block relative w-max">
            <button {onclick} {onblur} class="block font-body text-left text-white bg-nav hover:text-green-500 appearance-none py-1 px-4" >
            {count}{" "}{(*selected_item).as_str()}{" \u{23F7}"}
            </button>
            if *show_dropdown {

                <ul class="absolute top-12 py-2 px-2 shadow-2xl z-10 bg-nav inline-block w-auto">
                    {
                        props.items.clone().iter().map(|item| {
                            html!{<li onmousedown={on_item_click.clone()} class="cursor-pointer font-body text-white p-2 hover:text-green-500 block w-full">{ item.clone() }</li>}
                        }).collect::<Html>()
                    }              
            
                </ul>
            }
        </div>
        </>
    }
}