use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn UserSettings() -> Element {
    rsx! {
        p { "User Settings will be added here soon." }
    }
}
