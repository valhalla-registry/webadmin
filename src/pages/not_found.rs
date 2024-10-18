use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        for path in route {
            p { "{path}" }
        }
    }
}
