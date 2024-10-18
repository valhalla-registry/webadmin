use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "sticky top-0 w-full bg-white border-b py-2.5 ps-64",
            div {
                class: "flex justify-start px-5",
                "Header",
                "kshdfkssjd"
            }
        }
    }
}
