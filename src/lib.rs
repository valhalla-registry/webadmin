#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod layouts;
mod pages;

use crate::pages::index::Index;

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Index {},
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
