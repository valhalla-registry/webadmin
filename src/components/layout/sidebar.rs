use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::Route;

#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub name: String,
    pub route: String,
    pub children: Vec<MenuItem>
}

#[component]
pub fn Sidebar() -> Element {
    let items = vec![
        MenuItem {
            name: "Dashboard".into(),
            route: "/dashboard".into(),
            children: vec![]
        },
        MenuItem {
            name: "Settings".into(),
            route: "/settings".into(),
            children: vec![]
        }
    ];

    rsx! {
        div {
            class: "fixed top-0 start-0 bottom-0 w-64 bg-white border-e border-gray-200 pt-7 pb-10 overflow-y-auto",
            div {
                class: "px-6",
                "logo + name"
            }
            nav {
                class: "p-6 w-full flex flex-col flex-wrap",
                ul {
                    for item in items {
                        MenuItemComponent { item }
                    }
                }
            }
        }
    }
}

#[component]
fn MenuItemComponent(item: MenuItem) -> Element {
    if item.children.is_empty() {
        rsx! {
            li {
                Link { to: item.route, "{item.name}" }
            }
        }
    } else {
        rsx! {
            li {
                class: "hs-accordion",
                button {
                    class: "hs-accordion-toggle w-full text-start items-center gap-x-3.5 py-2 px-2.5",
                    "{item.name}"
                }
            }
        }
    }
}
