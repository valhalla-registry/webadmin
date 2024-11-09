pub mod performance;
pub mod overview;

use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        p { "dashboard" }
    }
}
