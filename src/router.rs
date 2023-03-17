use yew::{classes, function_component, html, Html};
use yew_router::prelude::*;

use crate::page::{About, Landing, News, NotFound};

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

struct RouteItem {
    route: AppRoute,
    label: &'static str,
}


#[function_component]
pub fn Nav() -> Html {
    let link_style = classes!(
        "hover:text-white",
        "text-xl",
        "px-3",
        "flex-initial",
        "transition",
        "ease-in-out",
        "delay-100",
        "duration-500",
        "text-white/70"
    );
    let route_items: Vec<RouteItem> = vec![
        RouteItem {
            route: AppRoute::Landing,
            label: "主页",
        },
        RouteItem {
            route: AppRoute::News,
            label: "新闻",
        },
        RouteItem {
            route: AppRoute::About,
            label: "关于我们",
        },
    ];
    html!(
        <>
            {
                for route_items.iter().map(|route| {
                    html! {
                            <Link<AppRoute> to={route.route.clone()} classes={link_style.clone()}>
                                {route.label}
                            </Link<AppRoute>>
                    }
                })
            }
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
