use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "mx-auto w-72",
            form {
                action: "/api/v1/auth/login",
                method: "get",
                input {}
                input {}
                button { "Submit" }
            }
        }
    }
}
