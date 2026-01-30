use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct LayoutProps{
    pub children: Children,
}

#[function_component()]
pub fn Layout(props: &LayoutProps) -> Html{
    html!{
        <div class="app-root">
            {for props.children.iter()}
        </div>
        }
}
