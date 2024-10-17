pub mod auth;

use leptos::*;
use leptos_meta::Body;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <Body class="bg-slate-800"/>
        <p> "index" </p>
    }
}