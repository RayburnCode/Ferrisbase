use dioxus::prelude::*;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Reports(id: String) -> Element {
    rsx! {
        p { "Reports Page" }
        p { "Display reports related to the project activities" }
        p { "Filter reports by date, type, or user" }
    }
    }
