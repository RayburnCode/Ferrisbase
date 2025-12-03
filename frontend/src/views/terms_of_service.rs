use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn TermsOfService() -> Element {
    rsx! {
        p { "Terms of Service will be added here soon." }
    }
}
