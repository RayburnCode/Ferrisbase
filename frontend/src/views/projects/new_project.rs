use dioxus::prelude::*;
use crate::components::projects::{NewProject};

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn CreateNewProject() -> Element {
    rsx! {

        NewProject {}
    }
    }
