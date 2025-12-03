use dioxus::prelude::*;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Authentication(id: String) -> Element {
    rsx! {
        p { "Authentication Page" }
        p { "Manage authentication settings for the project" }
    }
    }
