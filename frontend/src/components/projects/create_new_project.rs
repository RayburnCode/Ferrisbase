
use dioxus::prelude::*;


#[component]
pub fn NewProject() -> Element {
    rsx! {
        div { class: "max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-lg",
            // Header
            div { class: "mb-6",
                h2 { class: "text-3xl font-bold text-gray-900 mb-2", "Create a New Project" }
                p { class: "text-gray-600 leading-relaxed",
                    "Your project will have its own dedicated instance and full Postgres database. 
                    An API will be set up so you can easily interact with your new database."
                }
            }
            // Form
            form { class: "space-y-6",
                // Project Name Field
                div { class: "flex flex-col",
                    label {
                        class: "text-sm font-semibold text-gray-700 mb-2",
                        r#for: "project-name",
                        "Project Name"
                    }
                    input {
                        r#type: "text",
                        id: "project-name",
                        class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                        placeholder: "Enter project name",
                    }
                }
                // Database Password Field
                div { class: "flex flex-col",
                    label {
                        class: "text-sm font-semibold text-gray-700 mb-2",
                        r#for: "db-password",
                        "Database Password"
                    }
                    input {
                        r#type: "password",
                        id: "db-password",
                        class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                        placeholder: "Enter database password",
                    }
                }
                // Action Buttons
                div { class: "flex gap-4 pt-4",
                    button {
                        r#type: "submit",
                        class: "flex-1 bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-md transition duration-200 ease-in-out transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                        "Create Project"
                    }
                    button {
                        r#type: "button",
                        class: "flex-1 bg-gray-200 hover:bg-gray-300 text-gray-800 font-semibold py-3 px-6 rounded-md transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2",
                        "Cancel"
                    }
                }
            }
        }
    }

}
