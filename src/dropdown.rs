use yew::prelude::*;
use yew_hooks::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
    #[prop_or_default]
    pub items: Vec<AttrValue>,
    #[prop_or_default]
    pub on_selected: Callback<AttrValue>,
}

#[function_component]
pub fn Dropdown(props: &Props) -> Html {

    todo!()
}