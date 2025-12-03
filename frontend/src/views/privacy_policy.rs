use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn PrivacyPolicy() -> Element {
    rsx! {
        p { "Privacy Policy will be added here soon." }
    }
}
