use dioxus::prelude::*;
use crate::routes::Route;


#[component]
pub fn ProjectCard(
    project_name: String,
    database_status: String,
    created_at: String,
    slug: String,
    id: uuid::Uuid
) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow-md hover:shadow-xl transition-shadow duration-300 p-6 border border-gray-200",
            // Project Header
            div { class: "flex items-start justify-between mb-4",
                div { class: "flex-1",
                    h3 { class: "text-xl font-bold text-gray-900 mb-1", "{project_name}" }
                    p { class: "text-sm text-gray-500", "Created {created_at}" }
                }
                // Status Badge
                div { class: if database_status == "active" { "px-3 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded-full" } else if database_status == "inactive" { "px-3 py-1 bg-gray-100 text-gray-800 text-xs font-semibold rounded-full" } else { "px-3 py-1 bg-yellow-100 text-yellow-800 text-xs font-semibold rounded-full" },
                    "{database_status}"
                }
            }
            // Project Details
            div { class: "space-y-3 mb-6",
                div { class: "flex items-center text-sm",
                    span { class: "text-gray-600 font-medium w-24", "Database:" }
                    span { class: "text-gray-900", "PostgreSQL" }
                }
                div { class: "flex items-center text-sm",
                    span { class: "text-gray-600 font-medium w-24", "API Status:" }
                    span { class: "text-gray-900", "Ready" }
                }
            }
            // Action Buttons
            div { class: "flex gap-3 pt-4 border-t border-gray-200",
                Link {
                    to: Route::ProjectById {
                        id: slug.clone(),
                    },
                    class: "flex-1 bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 text-center",
                    "Open Project"
                }
                button { class: "bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium py-2 px-4 rounded-md transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2",
                    "Settings"
                }
            }
        }
    }

}
