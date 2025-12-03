use dioxus::prelude::*;

use crate::{components::layout::{UnprotectedLayout, ProjectLayout}, 
    views::{FAQ, Home, PrivacyPolicy, TermsOfService, NotFound}}; 
use crate::views::projects::{ProjectById, Projects, TableEditor, SQLEditor, 
    CreateNewProject, ProjectSettings, APIDocs, Logs, Reports, Authentication, Database};


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
        #[route("/terms-of-service")]
        TermsOfService {},
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
        #[route("/projects/:id/settings")]
        ProjectSettings {id: String},
        #[route("/projects/:id/api-docs")]
        APIDocs {id: String},
        #[route("/projects/:id/logs")]
        Logs {id: String},
        #[route("/projects/:id/reports")]
        Reports {id: String},
        #[route("/projects/:id/authentication")]
        Authentication {id: String},
        #[route("/projects/:id/database")]
        Database {id: String},

    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
