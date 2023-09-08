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

    let onblur = {
        let cloned_show_dropdown = show_dropdown.clone();

        Callback::from(move |_: FocusEvent| {
            cloned_show_dropdown.set(false);
        })
    };

    html!{
        <>
        <div class="inline-block relative">
            <button {onclick} {onblur} class="block font-body text-left text-white bg-nav hover:bg-green-950 appearance-none py-1 px-4" >
            {(*selected_item).as_str()}{" \u{23F7}"}
            </button>
            if *show_dropdown {

                <ul class="absolute py-4 px-2 shadow-2xl z-10 bg-nav">
                    {
                        props.items.clone().iter().map(|item| {
                            html!{<li class="cursor-pointer font-body text-white p-2 hover:bg-green-950 whitespace-no-wrap block">{ item.clone() }</li>}
                        }).collect::<Html>()
                    }              
            
                </ul>
            }
        </div>
        </>
    }
}