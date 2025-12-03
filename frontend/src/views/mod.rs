pub mod projects;
pub use projects::{Projects, ProjectById};


pub mod auth;

mod home;
pub use home::Home;



pub mod not_found;
pub use not_found::NotFound;

pub mod faq;
pub use faq::FAQ;

pub mod privacy_policy;
pub use privacy_policy::PrivacyPolicy;

pub mod terms_of_service;
pub use terms_of_service::TermsOfService;