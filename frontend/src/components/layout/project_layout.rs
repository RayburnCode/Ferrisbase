// client/components/layout/project_layout.rs
use dioxus::prelude::*;
use crate::Route;
use crate::components::projects::project_left_sidebar::ProjectLeftSidebar;
 
#[component]
pub fn ProjectLayout() -> Element {
    // Get the current route to extract the project slug/id
    let route = use_route::<Route>();
    
    // Extract the id from the current route
    let project_id = match route {
        Route::ProjectById { id } => id,
        Route::TableEditor { id } => id,
        Route::SQLEditor { id } => id,
        Route::ProjectSettings { id } => id,
        Route::APIDocs { id } => id,
        Route::Logs { id } => id,
        Route::Reports { id } => id,
        Route::Authentication { id } => id,
        Route::Database { id } => id,
        _ => String::from(""), // Fallback for non-project routes
    };
    
    rsx! {
        // Only render the sidebar and content area
        // Navbar and Footer are handled by SignedInLayout
        div { class: "flex gap-6",
            ProjectLeftSidebar { slug: project_id }
            div { class: "flex-1 bg-transparent", Outlet::<Route> {} }
        }
    }
}