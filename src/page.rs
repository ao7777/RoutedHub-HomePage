use yew::{function_component, html, Html};

use crate::component::landing::LetterBar;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <div class={"flex justify-center bg-rose-900 w-screen h-screen"}>
            <div class="grow bg-slate-900/30" />
            {
               for "RoutedHub".as_bytes().iter().enumerate().map(|(i,c)| html!{
                <LetterBar letter={c.clone() as char} index={i} />
               })
            }
            <div class="grow bg-slate-900/30" />
        </div>
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

#[function_component(News)]
pub fn news() -> Html {
    html! {
        <>
            <h1>{ "News" }</h1>
        </>
    }
}
