use yew::prelude::*;

#[component]
fn App() -> Html {

    html! {
        <div class={classes!("app-layout")}> 
        </div>
    }
}

fn main() {
    dotenv::dotenv().ok();
    yew::Renderer::<App>::new().render();
}
