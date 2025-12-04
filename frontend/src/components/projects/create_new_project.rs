use dioxus::prelude::*;
use crate::{Route, AuthState};
use crate::components::ErrorNotification;
use crate::hooks;

#[component]
pub fn NewProject() -> Element {
    let mut project_name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);
    let auth_state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    // Redirect if not authenticated
    use_effect(move || {
        if !auth_state.read().is_authenticated() {
            navigator.push(Route::LoginForm {});
        }
    });

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let name_val = project_name.read().clone();
        let desc_val = description.read().clone();

        // Validation
        if name_val.is_empty() {
            error_message.set(Some("Project name is required".to_string()));
            return;
        }

        if name_val.len() < 3 {
            error_message.set(Some("Project name must be at least 3 characters".to_string()));
            return;
        }

        error_message.set(None);
        is_loading.set(true);

        let create_project = hooks::use_create_project();
        let description_opt = if desc_val.is_empty() { None } else { Some(desc_val) };

        spawn(async move {
            match create_project(name_val, description_opt).await {
                Ok(project) => {
                    is_loading.set(false);
                    // Navigate to the new project
                    navigator.push(Route::ProjectById { id: project.slug });
                }
                Err(error) => {
                    is_loading.set(false);
                    error_message.set(Some(error));
                }
            }
        });
    };

    let handle_cancel = move |_| {
        navigator.push(Route::Projects {});
    };

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
            form { class: "space-y-6", onsubmit: handle_submit,

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
                        placeholder: "my-awesome-project",
                        required: true,
                        value: "{project_name}",
                        oninput: move |e| project_name.set(e.value()),
                    }
                    p { class: "mt-1 text-xs text-gray-500",
                        "Must be at least 3 characters. Only lowercase letters, numbers, and hyphens."
                    }
                }

                // Description Field
                div { class: "flex flex-col",
                    label {
                        class: "text-sm font-semibold text-gray-700 mb-2",
                        r#for: "description",
                        "Description (optional)"
                    }
                    textarea {
                        id: "description",
                        rows: "3",
                        class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                        placeholder: "What is this project for?",
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                    }
                }

                // Error message
                if let Some(msg) = &*error_message.read() {
                    ErrorNotification { message: msg.clone(), class: Some("mb-4".to_string()) }
                }

                // Action Buttons
                div { class: "flex gap-4 pt-4",
                    button {
                        r#type: "submit",
                        disabled: *is_loading.read(),
                        class: "flex-1 bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-md transition duration-200 ease-in-out transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-75 disabled:cursor-not-allowed disabled:transform-none",
                        if *is_loading.read() {
                            "Creating Project..."
                        } else {
                            "Create Project"
                        }
                    }
                    button {
                        r#type: "button",
                        onclick: handle_cancel,
                        disabled: *is_loading.read(),
                        class: "flex-1 bg-gray-200 hover:bg-gray-300 text-gray-800 font-semibold py-3 px-6 rounded-md transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 disabled:opacity-75",
                        "Cancel"
                    }
                }
            }
        }
    }
}
