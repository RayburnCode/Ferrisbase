
use dioxus::prelude::*;


#[component]
pub fn NewProject() -> Element {
    rsx! {
        p { "Create a new project" }
        p {
            "Your project will have its own dedicated instance and full Postgres database. 
            An API will be set up so you can easily interact with your new database."
        }
        p { "Project Name" }
        p { "Database Password" }


        p { "Create Project" }
        button { "Create Project" }
        button { "Cancel Project" }
    }

}
