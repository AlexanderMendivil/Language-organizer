use yew::prelude::*;

mod components;
use components::{layout::Layout};

#[component]
fn App() -> Html {

    html! {
        <Layout>
            <h1>{"hello"}</h1>
            <h2>{"second"}</h2>
        </Layout>
           }
}

fn main() {
    dotenv::dotenv().ok();
    yew::Renderer::<App>::new().render();
}
