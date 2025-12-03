use dioxus::prelude::*;
/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn SQLEditor(id: String) -> Element {

    
    rsx! {
        p { "SQL Editor Page" }
        p { "TextArea to be able to input SQL queries" }
        p { "Button to run the SQL query" }
        p { "Display area for query results" }
    }
}
