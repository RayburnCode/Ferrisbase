use dioxus::prelude::*;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Database(id: String) -> Element {
    rsx! {
        p { "Database Page" }
        p { "Display database related to the project" }
        p { "Manage database settings and configurations" }
    }
    }
