// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::Route;
use super::{Navbar, Footer};
use crate::components::projects::project_left_sidebar::ProjectLeftSidebar;
 
#[component]
pub fn ProjectLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {}
            main { class: "flex-1 bg-CustomBackground font-display text-MyText",
                // Layout container: sidebar + content
                div { class: "mx-auto px-4 sm:px-8 py-8 max-w-7xl",
                    div { class: "flex gap-6",
                        ProjectLeftSidebar {}
                        div { class: "flex-1 bg-transparent", Outlet::<Route> {} }
                    }
                }
            }
            Footer {}
        }
    }
}