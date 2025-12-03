use dioxus::prelude::*;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn APIDocs(id: String) -> Element {
    rsx! {
        p { "API Documentation Page" }
        p { "Display the auto-generated API documentation for the project" }

        p { "List of available endpoints" }
        p { "Details about each endpoint (method, URL, parameters, responses)" }
        p { "Example requests and responses" }
    }
    }
