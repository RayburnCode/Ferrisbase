use dioxus::prelude::*;


#[component]
pub fn ProjectLeftSidebar() -> Element {
    rsx! {
        nav { class: "w-64 bg-white border-r border-gray-200 min-h-screen p-4 hidden md:block",

            // Sidebar header
            div { class: "mb-6",
                h2 { class: "text-lg font-bold text-gray-900", "Projects" }
                p { class: "text-xs text-gray-500 mt-1", "Manage your project resources" }
            }

            // Project links
            ul { class: "space-y-1",
                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "🏷️" }
                        span { class: "text-sm font-medium", "Overview" }
                    }
                }
                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "🧾" }
                        span { class: "text-sm font-medium", "Table Editor" }
                    }
                }
                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "🧩" }
                        span { class: "text-sm font-medium", "SQL Editor" }
                    }
                }
                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "🗄️" }
                        span { class: "text-sm font-medium", "Database" }
                    }
                }
                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "🔐" }
                        span { class: "text-sm font-medium", "Authentication" }
                    }
                }

                li { class: "mt-3 pt-3 border-t border-gray-100" }

                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "📊" }
                        span { class: "text-sm font-medium", "Reports" }
                    }
                }
                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "📝" }
                        span { class: "text-sm font-medium", "Logs" }
                    }
                }
                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "📚" }
                        span { class: "text-sm font-medium", "API Docs" }
                    }
                }

                li { class: "mt-3 pt-3 border-t border-gray-100" }

                li {
                    a { class: "flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100",
                        span { class: "text-xl", "⚙️" }
                        span { class: "text-sm font-medium", "Settings" }
                    }
                }
            }
        }
    }
}
