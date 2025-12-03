use dioxus::prelude::*;
use crate::components::projects::{NewProject, ProjectCard};
/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn ProjectById() -> Element {

    
    rsx! {
        p { "Project Page" }
        p { "Button to add a new Project" }
        p { "Search and Filter Projects" }
    }
}
