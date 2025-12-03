use dioxus::prelude::*;


#[component]
pub fn StatisticCard(
    stat_type: String,
    request_count: String,
    change_percentage: String,
    is_increase: bool,
) -> Element {
    // Determine icon and color based on type
    let (icon, color_class, bg_class) = match stat_type.as_str() {
        "Database" => ("🗄️", "text-blue-600", "bg-blue-100"),
        "Auth" => ("🔐", "text-green-600", "bg-green-100"),
        "Storage" => ("💾", "text-purple-600", "bg-purple-100"),
        "API" => ("⚡", "text-orange-600", "bg-orange-100"),
        _ => ("📊", "text-gray-600", "bg-gray-100"),
    };

    rsx! {
        div { class: "bg-white rounded-lg shadow-md hover:shadow-lg transition-all duration-300 p-6 border border-gray-200",
            // Header with type and icon
            div { class: "flex items-center justify-between mb-4",
                div { class: "flex items-center gap-3",
                    div { class: "w-12 h-12 {bg_class} rounded-lg flex items-center justify-center text-2xl",
                        "{icon}"
                    }
                    h3 { class: "text-lg font-semibold text-gray-900", "{stat_type}" }
                }
            }
            // Request count
            div { class: "mb-4",
                p { class: "text-3xl font-bold text-gray-900", "{request_count}" }
                p { class: "text-sm text-gray-500 mt-1", "Total Requests" }
            }
            // Change indicator
            div { class: "flex items-center gap-2",
                span { class: if is_increase { "text-sm font-semibold text-green-600 flex items-center gap-1" } else { "text-sm font-semibold text-red-600 flex items-center gap-1" },
                    if is_increase {
                        "↑ +{change_percentage}%"
                    } else {
                        "↓ -{change_percentage}%"
                    }
                }
                span { class: "text-xs text-gray-500", "vs last period" }
            }
            // Mini histogram placeholder
            div { class: "mt-6 pt-4 border-t border-gray-200",
                div { class: "flex items-end justify-between h-16 gap-1",
                    // Simple bar chart representation
                    div { class: "flex-1 bg-gradient-to-t from-{color_class}/20 to-{color_class}/40 rounded-t h-10" }
                    div { class: "flex-1 bg-gradient-to-t from-{color_class}/20 to-{color_class}/40 rounded-t h-12" }
                    div { class: "flex-1 bg-gradient-to-t from-{color_class}/20 to-{color_class}/40 rounded-t h-8" }
                    div { class: "flex-1 bg-gradient-to-t from-{color_class}/20 to-{color_class}/40 rounded-t h-14" }
                    div { class: "flex-1 bg-gradient-to-t from-{color_class}/20 to-{color_class}/40 rounded-t h-16" }
                    div { class: "flex-1 bg-gradient-to-t from-{color_class}/20 to-{color_class}/40 rounded-t h-11" }
                    div { class: "flex-1 bg-gradient-to-t from-{color_class}/20 to-{color_class}/40 rounded-t h-13" }
                }
                p { class: "text-xs text-gray-500 text-center mt-2", "Last 7 days" }
            }
        }
    }
}
