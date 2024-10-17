#![allow(non_snake_case)]

mod pages;
mod components;

use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::components::layout::Layout;
use crate::pages::auth::Login;
use crate::pages::Index;

#[component]
pub fn App() -> impl IntoView {
    let authenticated = || true;

    view! {
        <Router>
            <Routes>
                <ProtectedRoute path="/settings" view=Layout condition=authenticated redirect_path="/auth/login">
                    <Route path="" view=|| view! {"settings index"} />
                </ProtectedRoute>
                <Route path="/" view=Index />
                <Route path="/auth/login" view=Login />
            </Routes>
        </Router>
    }
}
