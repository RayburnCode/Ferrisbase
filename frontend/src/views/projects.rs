use crate::components::Hero;
use dioxus::prelude::*;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Project() -> Element {
    rsx! {
        p { "Project Page" }
        p { "Button to add a new Project" }

        p { "Search and Filter Projects" }
    }
}
