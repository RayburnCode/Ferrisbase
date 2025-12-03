use crate::Route;
use dioxus::prelude::*;


#[component]
pub fn Navbar(children: Element) -> Element {
    let current_route = use_route::<Route>();
    let mut is_mobile_menu_open = use_signal(|| false);

    // Helper function to determine active class
    fn active_class(route: &Route, current_route: &Route, class: &str) -> String {
        if route == current_route {
            format!("{} text-blue-600 font-medium border-b-2 border-CustomHover", class)
        } else {
            class.to_string()
        }
    }

    // Helper function for mobile active class
    fn mobile_active_class(route: &Route, current_route: &Route, class: &str) -> String {
        if route == current_route {
            format!("{} text-blue-600 font-medium border-l-4 border-CustomHover bg-CustomHover bg-opacity-10", class)
        } else {
            class.to_string()
        }
    }

    rsx! {
        nav { class: "sticky top-0 z-50 w-full  bg-CustomNav backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    // Logo/Brand (you can add this later if needed)
                    div { class: "flex items-center md:hidden" }

                    // Desktop navigation (center)
                    div { class: "hidden md:flex items-center space-x-8",
                        Link {
                            to: Route::Home {},
                            class: active_class(
                                &Route::Home {},
                                &current_route,
                                "text-black hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Home"
                        }
                    }

                    // Mobile menu button and Contact Us button container
                    div { class: "flex items-center space-x-4",
                        // Mobile hamburger menu button
                        button {
                            class: "md:hidden inline-flex items-center justify-center p-2 rounded-md text-CustomAccent hover:text-CustomHover hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-CustomHover transition-colors",
                            onclick: move |_| {
                                is_mobile_menu_open.set(!is_mobile_menu_open());
                            },
                            // Hamburger icon
                            if !is_mobile_menu_open() {
                                svg {
                                    class: "h-6 w-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    stroke: "black",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M4 6h16M4 12h16M4 18h16",
                                    }
                                }
                            } else {
                                // Close icon
                                svg {
                                    class: "h-6 w-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    stroke: "black",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                }
                            }
                        }

                        // Contact Us button
                        Link {
                            to: Route::Home {},
                            class: if matches!(current_route, Route::Home {}) { "rounded-md bg-CustomHover px-4 py-2 text-sm font-medium hover:bg-green-800 text-CustomBackground shadow  focus:outline-none transition-colors" } else { "rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow hover:bg-green-800 focus:outline-none transition-colors" },
                            "Contact Us"
                        }
                    }
                }

                // Mobile menu
                if is_mobile_menu_open() {
                    div { class: "md:hidden border-t border-gray-200 bg-CustomNav",
                        div { class: "px-2 pt-2 pb-3 space-y-1",
                            Link {
                                to: Route::Home {},
                                class: mobile_active_class(
                                    &Route::Home {},
                                    &current_route,
                                    "block px-3 py-2 text-base font-medium text-black hover:text-CustomHover hover:bg-gray-50 transition-colors",
                                ),
                                onclick: move |_| is_mobile_menu_open.set(false),
                                "Home"
                            }
                        }
                    }
                }
            }
        }
    }
}