use yew::{classes, function_component, html, Html};
use yew_router::prelude::*;

use crate::page::{Landing, NotFound, About, News};

#[derive(Clone, Routable, PartialEq)]
enum AppRoute {
    #[at("/")]
    Landing,
    #[at("/about")]
    About,
    #[at("/news")]
    News,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn Nav() -> Html {
    let link_style = classes!("hover:text-white","text-xl","px-3","flex-initial","transition","ease-in-out","delay-100","duration-500","text-white/70");
    html!(
        <>
            <Link<AppRoute> to={AppRoute::Landing} classes={link_style.clone()}>
                { "主页" }
            </Link<AppRoute> >
            <Link<AppRoute> to={AppRoute::News} classes={link_style.clone()}>
                { "新闻" }
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
        AppRoute::News => html! { <News /> },
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
