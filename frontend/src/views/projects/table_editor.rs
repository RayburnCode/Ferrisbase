use dioxus::prelude::*;
/// The Project page component that will be rendered when the current route is `[Route::Project]`
#[component]
pub fn TableEditor() -> Element {

    
    rsx! {
        p { "Table Editor Page" }
        p { "Interface to edit table structure and data" }

        p { "Left side shows the different Schemas/Tables" }
        p { "Middle is a Excel-like grid to visualize the column and have the data." }
        p { "button to click and view the REST API URL to be able to post into the table" }
        p { "Row Level Security settings" }
    }
}
