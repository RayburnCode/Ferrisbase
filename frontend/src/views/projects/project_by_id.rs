use dioxus::prelude::*;
use crate::components::projects::statistic_card::StatisticCard;
use crate::{Route, AuthState};
use crate::hooks;
use shared::models::ProjectResponse;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn ProjectById(id: String) -> Element {
    let mut project = use_signal(|| None::<ProjectResponse>);
    let mut is_loading = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);
    let auth_state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    // Fetch project data on mount
    use_effect(move || {
        let slug = id.clone();
        
        // Redirect if not authenticated
        if !auth_state.read().is_authenticated() {
            navigator.push(Route::LoginForm {});
            return;
        }

        spawn(async move {
            let fetch_project = hooks::use_fetch_project();
            
            match fetch_project(slug).await {
                Ok(proj) => {
                    project.set(Some(proj));
                    is_loading.set(false);
                }
                Err(e) => {
                    error_message.set(Some(e));
                    is_loading.set(false);
                }
            }
        });
    });

    // Format date helper
    let format_date = |date_str: &str| -> String {
        // Simple date formatting - you can enhance this with chrono if needed
        date_str.split('T').next().unwrap_or(date_str).to_string()
    };

    rsx! {
        div { class: "min-h-screen bg-gray-50 p-8",
            if *is_loading.read() {
                // Loading state
                div { class: "max-w-7xl mx-auto",
                    div { class: "flex items-center justify-center py-12",
                        div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600" }
                    }
                }
            } else if let Some(err) = error_message.read().as_ref() {
                // Error state
                div { class: "max-w-7xl mx-auto",
                    div { class: "bg-red-50 border border-red-200 rounded-lg p-6",
                        h2 { class: "text-xl font-bold text-red-900 mb-2", "Error Loading Project" }
                        p { class: "text-red-700", "{err}" }
                        div { class: "mt-4",
                            Link {
                                to: Route::Projects {},
                                class: "text-blue-600 hover:text-blue-700 font-medium",
                                "← Back to Projects"
                            }
                        }
                    }
                }
            } else if let Some(proj) = project.read().as_ref() {
                // Project loaded
                div { class: "max-w-7xl mx-auto mb-8",
                    div { class: "flex items-center justify-between",
                        div {
                            div { class: "flex items-center gap-3 mb-2",
                                Link {
                                    to: Route::Projects {},
                                    class: "text-gray-500 hover:text-gray-700",
                                    "← Projects"
                                }
                            }
                            h1 { class: "text-4xl font-bold text-gray-900 mb-2", "{proj.name}" }
                            if let Some(ref desc) = proj.description {
                                p { class: "text-gray-600", "{desc}" }
                            } else {
                                p { class: "text-gray-600", "Monitor and manage your project" }
                            }
                        }
                        div { class: "flex items-center gap-2",
                            span { class: if proj.database_status == "active" { "px-4 py-2 bg-green-100 text-green-800 text-sm font-semibold rounded-full" } else { "px-4 py-2 bg-gray-100 text-gray-800 text-sm font-semibold rounded-full" },
                                "{proj.database_status}"
                            }
                        }
                    }
                }

                // Statistics Grid
                div { class: "max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                    StatisticCard {
                        stat_type: "Database".to_string(),
                        request_count: "0".to_string(),
                        change_percentage: "0".to_string(),
                        is_increase: true,
                    }
                    StatisticCard {
                        stat_type: "Auth".to_string(),
                        request_count: "0".to_string(),
                        change_percentage: "0".to_string(),
                        is_increase: true,
                    }
                    StatisticCard {
                        stat_type: "Storage".to_string(),
                        request_count: "0".to_string(),
                        change_percentage: "0".to_string(),
                        is_increase: false,
                    }
                    StatisticCard {
                        stat_type: "API".to_string(),
                        request_count: "0".to_string(),
                        change_percentage: "0".to_string(),
                        is_increase: true,
                    }
                }

                // Project Details Section
                div { class: "max-w-7xl mx-auto mt-8 bg-white rounded-lg shadow-md p-6",
                    h2 { class: "text-2xl font-bold text-gray-900 mb-4", "Project Details" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                        div { class: "border-l-4 border-blue-500 pl-4",
                            p { class: "text-sm text-gray-600", "Project ID" }
                            p { class: "text-lg font-semibold text-gray-900 font-mono",
                                "{proj.id}"
                            }
                        }
                        div { class: "border-l-4 border-purple-500 pl-4",
                            p { class: "text-sm text-gray-600", "Slug" }
                            p { class: "text-lg font-semibold text-gray-900", "{proj.slug}" }
                        }
                        div { class: "border-l-4 border-indigo-500 pl-4",
                            p { class: "text-sm text-gray-600", "Database Type" }
                            p { class: "text-lg font-semibold text-gray-900", "PostgreSQL" }
                        }
                        div { class: "border-l-4 border-green-500 pl-4",
                            p { class: "text-sm text-gray-600", "Status" }
                            p { class: if proj.database_status == "active" { "text-lg font-semibold text-green-600" } else { "text-lg font-semibold text-gray-600" },
                                "{proj.database_status}"
                            }
                        }
                        div { class: "border-l-4 border-orange-500 pl-4",
                            p { class: "text-sm text-gray-600", "Created" }
                            p { class: "text-lg font-semibold text-gray-900",
                                "{format_date(&proj.created_at)}"
                            }
                        }
                        if let Some(ref updated) = proj.updated_at {
                            div { class: "border-l-4 border-yellow-500 pl-4",
                                p { class: "text-sm text-gray-600", "Last Updated" }
                                p { class: "text-lg font-semibold text-gray-900",
                                    "{format_date(updated)}"
                                }
                            }
                        }
                    }
                }

                // Actions Section
                div { class: "max-w-7xl mx-auto mt-8 bg-white rounded-lg shadow-md p-6",
                    h2 { class: "text-2xl font-bold text-gray-900 mb-4", "Quick Actions" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        Link {
                            to: Route::ProjectSettings {
                                id: proj.slug.clone(),
                            },
                            class: "flex items-center justify-center gap-2 px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition",
                            "⚙️ Settings"
                        }
                        button { class: "flex items-center justify-center gap-2 px-4 py-3 bg-gray-200 hover:bg-gray-300 text-gray-800 font-medium rounded-lg transition",
                            "📊 View Analytics"
                        }
                        button { class: "flex items-center justify-center gap-2 px-4 py-3 bg-gray-200 hover:bg-gray-300 text-gray-800 font-medium rounded-lg transition",
                            "🔗 API Documentation"
                        }
                    }
                }
            }
        }
    }
}
