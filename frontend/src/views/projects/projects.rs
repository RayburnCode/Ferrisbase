use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::projects::{ProjectCard};
/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-8",
                div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6",
                    div {
                        h1 { class: "text-4xl font-bold text-gray-900", "My Projects" }
                        p { class: "text-gray-600 mt-2",
                            "Manage and monitor all your Ferrisbase projects"
                        }
                    }
                    Link {
                        to: Route::CreateNewProject {},
                        class: "inline-flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold px-6 py-3 rounded-md shadow-sm transition duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                        span { class: "text-lg", "+" }
                        "Add New Project"
                    }
                }
                // Search and Filter Section
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    div { class: "flex flex-col sm:flex-row gap-4",
                        input {
                            r#type: "text",
                            placeholder: "Search projects...",
                            class: "flex-1 px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                        }
                        select { class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                            option { "All Status" }
                            option { "Active" }
                            option { "Inactive" }
                            option { "Pending" }
                        }
                        select { class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                            option { "Sort by: Newest" }
                            option { "Sort by: Oldest" }
                            option { "Sort by: Name" }
                        }
                    }
                }
            }
            // Project Cards Grid
            div { class: "max-w-7xl mx-auto",
                h2 { class: "text-2xl font-semibold text-gray-900 mb-4", "Your Projects" }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    ProjectCard {
                        id: uuid::Uuid::new_v4(),
                        project_name: "Example Project".to_string(),
                        database_status: "active".to_string(),
                        created_at: "2024-01-01".to_string(),
                        slug: "example-project".to_string(),
                    }
                    ProjectCard {
                        id: uuid::Uuid::new_v4(),
                        project_name: "Production API".to_string(),
                        database_status: "active".to_string(),
                        created_at: "2024-02-15".to_string(),
                        slug: "production-api".to_string(),
                    }
                    ProjectCard {
                        id: uuid::Uuid::new_v4(),
                        project_name: "Dev Environment".to_string(),
                        database_status: "inactive".to_string(),
                        created_at: "2024-03-10".to_string(),
                        slug: "dev-environment".to_string(),
                    }
                }
            }
        }
    }
}
