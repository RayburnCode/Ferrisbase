use crate::Route;
use dioxus::prelude::*;


#[component]
pub fn Navbar(children: Element) -> Element {
    const LOGO: Asset = asset!("/assets/Original_Ferris.svg");

    let mut is_mobile_menu_open = use_signal(|| false);

    rsx! {
        nav { class: "sticky top-0 z-50 w-full bg-white backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    // Logo
                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-2 text-xl font-bold",
                        img { class: "w-8 h-8", src: "{LOGO}" }
                        span { class: "text-gray-900", "Ferrisbase" }
                    }

                    // Desktop navigation
                    div { class: "hidden md:flex items-center gap-8",
                        Link {
                            to: Route::FAQ {},
                            class: "text-sm font-medium text-gray-700 hover:text-blue-600 transition-colors",
                            "FAQ"
                        }
                        // Auth buttons
                        div { class: "flex items-center gap-3",
                            Link {
                                to: Route::LoginForm {},
                                class: "px-4 py-2 text-sm font-medium text-gray-700 hover:text-gray-900 transition-colors",
                                "Log in"
                            }
                            Link {
                                to: Route::RegisterForm {},
                                class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition-colors shadow-sm",
                                "Get Started"
                            }
                        }
                    }

                    // Mobile menu button
                    button {
                        class: "md:hidden inline-flex items-center justify-center p-2 rounded-lg text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors",
                        onclick: move |_| is_mobile_menu_open.set(!is_mobile_menu_open()),
                        if !is_mobile_menu_open() {
                            svg {
                                class: "h-6 w-6",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "M4 6h16M4 12h16M4 18h16",
                                }
                            }
                        } else {
                            svg {
                                class: "h-6 w-6",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "M6 18L18 6M6 6l12 12",
                                }
                            }
                        }
                    }
                }
            }
            // Mobile menu
            if is_mobile_menu_open() {
                div { class: "md:hidden border-t border-gray-200 bg-white",
                    div { class: "px-4 py-3 space-y-2",
                        Link {
                            to: Route::FAQ {},
                            class: "block px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg",
                            onclick: move |_| is_mobile_menu_open.set(false),
                            "FAQ"
                        }
                    }
                    // Mobile auth section
                    div { class: "px-4 py-3 border-t border-gray-200 space-y-2",
                        Link {
                            to: Route::LoginForm {},
                            class: "block text-center w-full px-4 py-2 text-sm font-medium text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50",
                            onclick: move |_| is_mobile_menu_open.set(false),
                            "Log in"
                        }
                        Link {
                            to: Route::RegisterForm {},
                            class: "block text-center w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700",
                            onclick: move |_| is_mobile_menu_open.set(false),
                            "Get Started"
                        }
                    }
                }
            }
        }
    }
}