use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    rsx! {
        p {
            class: "fg-red-100",
            "Hello World!"
        }
    }
}