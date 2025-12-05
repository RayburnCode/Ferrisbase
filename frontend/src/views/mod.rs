pub mod projects;


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

pub mod user_settings;
pub use user_settings::UserSettings;