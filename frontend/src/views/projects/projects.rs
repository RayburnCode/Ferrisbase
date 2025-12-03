use dioxus::prelude::*;
use crate::components::projects::{NewProject, ProjectCard};
/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Projects() -> Element {

    
    rsx! {
        p { "Project Page" }
        p { "Button to add a new Project" }
        NewProject {}
        p { "Search and Filter Projects" }

        p { "Project Cards" }
        ProjectCard {
            project_name: "Example Project".to_string(),
            database_status: "active".to_string(),
            created_at: "2024-01-01".to_string(),
        }
    }
}
