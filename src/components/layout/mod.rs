mod header;
mod sidebar;

use dioxus::prelude::*;

use crate::components::layout::header::Header;
use crate::components::layout::sidebar::Sidebar;
use crate::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        body {
            class: "bg-gray-100 h-screen min-h-fit",
            Header {},
            Sidebar {},
            div {
                class: "w-full pt-10 px-4 ps-72",
                Outlet::<Route> {}
            }
        }
    }
}
