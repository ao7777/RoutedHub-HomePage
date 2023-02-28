use crate::{
    component::{footer::Footer, header::Header},
    router::Body,
};
use yew::{function_component, html, Html};
use yew_router::BrowserRouter;
mod component;
mod page;
mod router;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter >
            <Header />
            <Body />
            <Footer />
        </BrowserRouter >
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
