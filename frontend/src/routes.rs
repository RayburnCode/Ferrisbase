use dioxus::prelude::*;

use crate::{components::layout::{UnprotectedLayout, ProjectLayout}, 
    views::{FAQ, Home, PrivacyPolicy, TermsOfService, NotFound}}; 
use crate::views::projects::{ProjectById, Projects, TableEditor, SQLEditor, CreateNewProject};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(UnprotectedLayout)]
        #[route("/")]
        Home {},
        #[route("/faq")]
        FAQ {},
        #[route("/privacy-policy")]
        PrivacyPolicy {},
        #[route("/projects")]
        Projects {},
        #[route("/projects/new")]
        CreateNewProject {},
    #[end_layout]


    #[layout(ProjectLayout)]
        #[route("/projects/:id")]
        ProjectById {id: String},
        #[route("/projects/:id/table-editor")]
        TableEditor {id: String},
        #[route("/projects/:id/sql-editor")]
        SQLEditor {id: String},
        #[route("/terms-of-service")]
        TermsOfService {},
    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
