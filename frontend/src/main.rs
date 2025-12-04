use dioxus::prelude::*;


mod auth;
mod components;
mod views;
mod hooks;
pub mod routes;
pub use routes::Route;
pub use auth::{AuthState, User};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    // Initialize auth state from localStorage
    use_context_provider(|| Signal::new(AuthState::load_from_storage()));
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
