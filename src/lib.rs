#![allow(non_snake_case)]

mod pages;
mod components;

use dioxus::prelude::*;

use crate::components::layout::Layout;
// use crate::pages::auth::Login;
use crate::pages::Home;
use crate::pages::auth::Login;
use crate::pages::not_found::NotFound;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home,
    #[end_layout]

    #[route("/auth/login")]
    Login,

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

pub fn app() -> Element {
    rsx! {
        style { {include_str!("../assets/tailwind.css")} }
        Router::<Route> {}
    }
}
