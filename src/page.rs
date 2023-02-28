use yew::{function_component, html, Html};

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <>
            <h1>{ "Landing" }</h1>
        </>
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <h1>{ "404" }</h1>
        </>
    }
}

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <h1>{ "About" }</h1>
        </>
    }
}