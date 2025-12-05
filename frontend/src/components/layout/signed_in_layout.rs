// client/components/layout/signed_in_layout.rs
use dioxus::prelude::*;
use crate::Route;
use super::{Navbar, Footer};
 
#[component]
pub fn SignedInLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {}
            main { class: "flex-1 bg-CustomBackground font-display text-MyText",
                div { class: "px-4 sm:px-8 py-8", Outlet::<Route> {} }
            }
            Footer {}
        }
    }
}