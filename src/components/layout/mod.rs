mod header;
mod sidebar;

use leptos::*;
use leptos_meta::Body;
use leptos_router::Outlet;
use crate::components::layout::header::Header;
use crate::components::layout::sidebar::Sidebar;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <Body class="bg-slate-900" />
        <Header />
        <Sidebar />
        <Outlet />
    }
}