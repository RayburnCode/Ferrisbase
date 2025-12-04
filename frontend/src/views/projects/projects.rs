use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::projects::ProjectCard;
use crate::{AuthState, hooks};
use shared::models::ProjectResponse;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn Projects() -> Element {
    let mut projects = use_signal(|| Vec::<ProjectResponse>::new());
    let mut is_loading = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);
    let mut search_query = use_signal(|| String::new());
    let mut status_filter = use_signal(|| "all".to_string());
    let auth_state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    // Fetch projects on mount
    use_effect(move || {
        // Redirect if not authenticated
        if !auth_state.read().is_authenticated() {
            navigator.push(Route::LoginForm {});
            return;
        }

        spawn(async move {
            let fetch_projects = hooks::use_fetch_projects();
            
            match fetch_projects().await {
                Ok(projs) => {
                    projects.set(projs);
                    is_loading.set(false);
                }
                Err(e) => {
                    error_message.set(Some(e));
                    is_loading.set(false);
                }
            }
        });
    });

    // Filter projects based on search and status
    let filtered_projects = move || {
        let search = search_query.read().to_lowercase();
        let status = status_filter.read().clone();
        
        projects.read()
            .iter()
            .filter(|p| {
                // Filter by search query
                let matches_search = search.is_empty() || 
                    p.name.to_lowercase().contains(&search) ||
                    p.slug.to_lowercase().contains(&search) ||
                    p.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&search));
                
                // Filter by status
                let matches_status = status == "all" || p.database_status.to_lowercase() == status.to_lowercase();
                
                matches_search && matches_status
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    // Format date helper
    let format_date = |date_str: &str| -> String {
        date_str.split('T').next().unwrap_or(date_str).to_string()
    };

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
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value()),
                        }
                        select {
                            class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                            value: "{status_filter}",
                            onchange: move |e| status_filter.set(e.value()),
                            option { value: "all", "All Status" }
                            option { value: "active", "Active" }
                            option { value: "inactive", "Inactive" }
                            option { value: "pending", "Pending" }
                        }
                    }
                }
            }

            // Project Cards Grid
            div { class: "max-w-7xl mx-auto",
                if *is_loading.read() {
                    // Loading state
                    div { class: "flex items-center justify-center py-12",
                        div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600" }
                    }
                } else if let Some(err) = error_message.read().as_ref() {
                    // Error state
                    div { class: "bg-red-50 border border-red-200 rounded-lg p-6",
                        h2 { class: "text-xl font-bold text-red-900 mb-2", "Error Loading Projects" }
                        p { class: "text-red-700", "{err}" }
                    }
                } else if filtered_projects().is_empty() {
                    // Empty state
                    div { class: "text-center py-12",
                        div { class: "text-gray-400 text-6xl mb-4", "📁" }
                        h2 { class: "text-2xl font-semibold text-gray-900 mb-2",
                            if projects.read().is_empty() {
                                "No projects yet"
                            } else {
                                "No matching projects"
                            }
                        }
                        p { class: "text-gray-600 mb-6",
                            if projects.read().is_empty() {
                                "Get started by creating your first project"
                            } else {
                                "Try adjusting your search or filters"
                            }
                        }
                        if projects.read().is_empty() {
                            Link {
                                to: Route::CreateNewProject {},
                                class: "inline-flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold px-6 py-3 rounded-md shadow-sm transition",
                                span { class: "text-lg", "+" }
                                "Create Your First Project"
                            }
                        }
                    }
                } else {
                    h2 { class: "text-2xl font-semibold text-gray-900 mb-4",
                        "Your Projects ({filtered_projects().len()})"
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        for project in filtered_projects() {
                            ProjectCard {
                                key: "{project.id}",
                                id: uuid::Uuid::parse_str(&project.id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
                                project_name: project.name.clone(),
                                database_status: project.database_status.clone(),
                                created_at: format_date(&project.created_at),
                                slug: project.slug.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}
