// client/components/layout/signed_in_layout.rs
use dioxus::prelude::*;
use crate::{Route, AuthState};
use super::{SignedInNavbar, Footer};
 
#[component]
pub fn SignedInLayout() -> Element {
    let auth_state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    // Redirect to login if not authenticated
    use_effect(move || {
        if !auth_state.read().is_authenticated() {
            navigator.push(Route::LoginForm {});
        }
    });

    // Don't render layout if not authenticated
    if !auth_state.read().is_authenticated() {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div { class: "flex flex-col min-h-screen",
            SignedInNavbar {}
            main { class: "flex-1 bg-CustomBackground font-display text-MyText",
                div { class: "px-4 sm:px-8 py-8", Outlet::<Route> {} }
            }
            Footer {}
        }
    }
}