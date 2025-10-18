// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::Route;
use super::{Footer};

#[component]
pub fn UnprotectedLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: "flex-1 bg-CustomBackground font-display text-MyText",
                div { class: "mx-auto px-6 sm:px-8 py-8", Outlet::<Route> {} }
            }
            Footer {}
        }
    }
}