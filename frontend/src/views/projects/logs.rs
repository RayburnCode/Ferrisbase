use dioxus::prelude::*;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Logs(id: String) -> Element {
    rsx! {
        p { "Logs Page" }
        p { "Display logs related to the project activities" }
        p { "Filter logs by date, type, or user" }
    }
    }
