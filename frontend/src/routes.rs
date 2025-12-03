use dioxus::prelude::*;

use crate::{components::layout::UnprotectedLayout, 
    views::{FAQ, Home, PrivacyPolicy, Projects, TermsOfService}};


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
        #[route("/terms-of-service")]
        TermsOfService {},
}
