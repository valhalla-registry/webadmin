use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            class: "fixed top-0 start-0 bottom-0 w-64 bg-white border-e border-gray-200 pt-7 pb-10 overflow-y-auto",
            "content more"
            p {
                class: "bg-red-500",
                "content 123sdfsdf4"
            }
        }
    }
}
