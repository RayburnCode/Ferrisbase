use dioxus::prelude::*;
/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn ProjectSettings(id: String) -> Element {

    
    rsx! {
        p { "Project Settings Page" }
        p { "Interface to edit project settings" }

        p { "Be able to change the name of the Project" }
        p { "Add additional users to the Project" }
        p { "Manage project permissions and roles" }

        h1 { "API Settings" }

        p {
            "Adding a TLD to the project(project should autogenerate a hash that can be used for API calls)"
        }
        p { "list Project URL" }
        p { "Generate API Keys" }
    }
}
