use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn FAQ() -> Element {
    rsx! {
        p { "Frequently Asked Questions will be added here soon." }
    }
}
