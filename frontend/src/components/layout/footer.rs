use dioxus::prelude::*;
use crate::routes::Route;


#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "  w-full bg-gray-900 text-gray-200 py-6 border-t border-gray-700",
            div { class: " mx-auto px-6",
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-8 mb-8",
                    // Company Info
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-semibold text-white", "Ferrisbase" }
                        p { class: "text-sm text-gray-400 leading-relaxed",
                            "Rusty solutions for modern applications. Building robust and efficient systems with the power of Rust."
                        }
                    }
                    // Quick Links
                    div { class: "space-y-4" }
                    // Support
                    div { class: "space-y-4",
                        h4 { class: "text-md font-medium text-white", "Support" }
                        div { class: "space-y-2",
                            Link {
                                class: "block text-sm text-gray-400 hover:text-blue-400 transition-colors",
                                to: Route::FAQ {},
                                "FAQ"
                            }
                            Link {
                                class: "block text-sm text-gray-400 hover:text-blue-400 transition-colors",
                                to: Route::PrivacyPolicy {},
                                "Privacy Policy"
                            }

                            Link {
                                class: "block text-sm text-gray-400 hover:text-blue-400 transition-colors",
                                to: Route::TermsOfService {},
                                "Terms of Service"
                            }
                        }
                    }
                    // Contact & Social
                    div { class: "space-y-4",
                        h4 { class: "text-md font-medium text-white", "Get In Touch" }
                        div { class: "space-y-3",
                            a {
                                class: "flex items-center text-sm text-gray-400 hover:text-blue-400 transition-colors",
                                href: "mailto:team@ferrisbase.com",
                                span { class: "mr-2", "✉" }
                                "team@ferrisbase.com"
                            }
                            a {
                                class: "flex items-center text-sm text-gray-400 hover:text-blue-400 transition-colors",
                                href: "tel:+1-858-252-3052",
                                span { class: "mr-2", "📞" }
                                "(858) 252-3052"
                            }
                            div { class: "flex space-x-4 mt-4",
                                a {
                                    class: "text-gray-400 hover:text-blue-400 transition-colors",
                                    href: "#",
                                    "LinkedIn"
                                }
                                a {
                                    class: "text-gray-400 hover:text-blue-400 transition-colors",
                                    href: "#",
                                    "X"
                                }
                                a {
                                    class: "text-gray-400 hover:text-blue-400 transition-colors",
                                    href: "#",
                                    "YouTube"
                                }
                            }
                        }
                    }
                }
            }
            // Bottom Bar
            div { class: "pt-2 max-w-7xl mx-auto  border-t border-gray-700 flex flex-col md:flex-row justify-between items-center",
                div { class: "mx-6 text-sm text-gray-400 mb-4 md:mb-0",
                    "© 2025 Ferrisbase. All rights reserved."
                }
            }
        }
    }
    }
