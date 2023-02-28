use chrono::Datelike;

use yew::{function_component, html, Html};

#[function_component]
pub fn Footer() -> Html {
    let the_year = chrono::Utc::now().year();
    html! {
        <footer class="bg-rose-900 py-2 px-1 text-white text-center">
            <p>{ "上海交通大学学生管理委员会网络技术部©"}{the_year}</p>
        </footer>
    }
}
