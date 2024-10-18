pub mod not_found;
pub mod settings;
pub mod auth;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        // card
        div {
            class: "max-w-[85rem] px-4 py-2 sm:px-6 lg:px-8 lg:py-2 mx-auto",
            div {
                class: "bg-white border shadow-sm rounded-xl",
                div {
                    class: "p-4",
                    "card content"
                }
            }
        }
    }
}