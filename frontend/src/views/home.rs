use dioxus::prelude::*;
use crate::routes::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 flex items-center",

            div { class: "max-w-7xl mx-auto px-6 py-16 grid grid-cols-1 lg:grid-cols-2 gap-12 items-center",

                // Left: Hero
                div { class: "lg:pr-8",
                    h1 { class: "text-5xl sm:text-6xl font-extrabold text-gray-900 leading-tight",
                        "Created to Scale"
                    }
                    p { class: "mt-6 text-lg text-gray-600 max-w-xl",
                        "Spin up a dedicated Postgres instance with an API and authentication baked in. Ferrisbase lets you get production-ready projects running quickly so you can focus on building features."
                    }

                    // Features list
                    ul { class: "mt-8 grid grid-cols-1 sm:grid-cols-2 gap-4",
                        li { class: "flex items-start gap-3",
                            span { class: "text-blue-600 text-xl", "✅" }
                            span { class: "text-sm text-gray-700",
                                "Postgres Database with dedicated schema and backups"
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "text-green-600 text-xl", "🔐" }
                            span { class: "text-sm text-gray-700",
                                "Authentication and user management out-of-the-box"
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "text-orange-500 text-xl", "⚡" }
                            span { class: "text-sm text-gray-700",
                                "Auto-generated REST APIs for your data models"
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "text-purple-600 text-xl", "💾" }
                            span { class: "text-sm text-gray-700",
                                "Scalable storage and deployment options"
                            }
                        }
                    }

                    // Actions
                    div { class: "mt-8 flex flex-wrap gap-4",
                        Link {
                            to: Route::LoginForm {},
                            id: "default".to_string(),
                        }
                        span { class: "inline-flex items-center justify-center bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-md transition duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500",
                            "Get Started"
                        }
                        button { class: "inline-flex items-center justify-center bg-white border border-gray-200 hover:bg-gray-50 text-gray-800 font-medium py-3 px-6 rounded-md transition duration-200",
                            "Learn More"
                        }
                    }
                }

                // Right: Visual / placeholder card
                div { class: "flex justify-center",
                    div { class: "w-full max-w-md bg-white rounded-2xl shadow-lg border border-gray-200 p-6",
                        h3 { class: "text-lg font-semibold text-gray-900 mb-2", "Example Project" }
                        p { class: "text-sm text-gray-600 mb-4",
                            "A sample project preview showing stats and quick actions."
                        }

                        div { class: "grid grid-cols-2 gap-3",
                            div { class: "p-3 bg-gray-50 rounded-md text-center",
                                p { class: "text-xs text-gray-500", "DB Size" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "12.4 GB"
                                }
                            }
                            div { class: "p-3 bg-gray-50 rounded-md text-center",
                                p { class: "text-xs text-gray-500", "Requests/day" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "18.9k"
                                }
                            }
                            div { class: "p-3 bg-gray-50 rounded-md text-center",
                                p { class: "text-xs text-gray-500", "Active Users" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "3.8k"
                                }
                            }
                            div { class: "p-3 bg-gray-50 rounded-md text-center",
                                p { class: "text-xs text-gray-500", "Region" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "us-east-1"
                                }
                            }
                        }

                        div { class: "mt-6 flex gap-3",
                            button { class: "flex-1 bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-md text-sm font-medium",
                                "Open"
                            }
                            button { class: "flex-1 bg-gray-100 hover:bg-gray-200 text-gray-800 py-2 px-4 rounded-md text-sm font-medium",
                                "Settings"
                            }
                        }
                    }
                }
            }
        }
    }
}
