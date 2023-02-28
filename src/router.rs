use yew::{classes, function_component, html, Html};
use yew_router::prelude::*;

use crate::page::{Landing, NotFound, About};

#[derive(Clone, Routable, PartialEq)]
enum AppRoute {
    #[at("/")]
    Landing,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn Nav() -> Html {
    let link_style = classes!("hover:text-white/70","text-2xl","px-3","leading-loose","flex-initial");
    html!(
        <>
            <Link<AppRoute> to={AppRoute::Landing} classes={link_style.clone()}>
                { "主页" }
            </Link<AppRoute> >
            <Link<AppRoute> to={AppRoute::About} classes={link_style}>
                { "关于我们" }
            </Link<AppRoute> >
        </>
    )
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Landing => html! { <Landing /> },
        AppRoute::About => html! { <About /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component(Body)]
pub fn body() -> Html {
    html! {
            <main class="h-screen">
                <Switch<AppRoute> render={switch} />
            </main>
    }
}
