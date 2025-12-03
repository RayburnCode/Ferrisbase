use dioxus::prelude::*;
use crate::components::projects::statistic_card::StatisticCard;

/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn ProjectById(id: String) -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-8",
            // Page Header
            div { class: "max-w-7xl mx-auto mb-8",
                div { class: "flex items-center justify-between",
                    div {
                        h1 { class: "text-4xl font-bold text-gray-900 mb-2", "Project: {id}" }
                        p { class: "text-gray-600", "Monitor and manage your project statistics" }
                    }
                }
            }
            // Statistics Grid
            div { class: "max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                StatisticCard {
                    stat_type: "Database".to_string(),
                    request_count: "12,543".to_string(),
                    change_percentage: "12.5".to_string(),
                    is_increase: true,
                }
                StatisticCard {
                    stat_type: "Auth".to_string(),
                    request_count: "3,891".to_string(),
                    change_percentage: "8.2".to_string(),
                    is_increase: true,
                }
                StatisticCard {
                    stat_type: "Storage".to_string(),
                    request_count: "7,234".to_string(),
                    change_percentage: "3.1".to_string(),
                    is_increase: false,
                }
                StatisticCard {
                    stat_type: "API".to_string(),
                    request_count: "18,942".to_string(),
                    change_percentage: "15.7".to_string(),
                    is_increase: true,
                }
            }
            // Additional Project Details Section
            div { class: "max-w-7xl mx-auto mt-8 bg-white rounded-lg shadow-md p-6",
                h2 { class: "text-2xl font-bold text-gray-900 mb-4", "Project Details" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "border-l-4 border-blue-500 pl-4",
                        p { class: "text-sm text-gray-600", "Database Type" }
                        p { class: "text-lg font-semibold text-gray-900", "PostgreSQL" }
                    }
                    div { class: "border-l-4 border-green-500 pl-4",
                        p { class: "text-sm text-gray-600", "Status" }
                        p { class: "text-lg font-semibold text-green-600", "Active" }
                    }
                    div { class: "border-l-4 border-purple-500 pl-4",
                        p { class: "text-sm text-gray-600", "Region" }
                        p { class: "text-lg font-semibold text-gray-900", "US-East-1" }
                    }
                    div { class: "border-l-4 border-orange-500 pl-4",
                        p { class: "text-sm text-gray-600", "Created" }
                        p { class: "text-lg font-semibold text-gray-900", "Nov 15, 2025" }
                    }
                }
            }
        }
    }
}
