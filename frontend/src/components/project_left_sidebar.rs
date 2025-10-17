use dioxus::prelude::*;


#[component]
pub fn ProjectLeftSidebar() -> Element {
    rsx! {
        div { "Project Left Sidebar" }
        ul {
            li { "Overview" }
            li { "Table Editor" }
            li { "SQL Editor" }
            li { "Database" }
            li { "Authentication" }
            li { "Storage" }

            li { "Reports" }
            li { "Logs" }
            li { "API Docs" }

            li { "Settings" }
        }
    }
}
