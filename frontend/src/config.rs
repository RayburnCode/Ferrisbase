/// API configuration
pub const API_BASE_URL: &str = "http://127.0.0.1:3000";

/// API endpoints
pub mod endpoints {
    use super::API_BASE_URL;

    // Auth endpoints
    pub fn login() -> String {
        format!("{}/api/auth/login", API_BASE_URL)
    }

    pub fn register() -> String {
        format!("{}/api/auth/register", API_BASE_URL)
    }

    pub fn logout() -> String {
        format!("{}/api/auth/logout", API_BASE_URL)
    }

    pub fn me() -> String {
        format!("{}/api/auth/me", API_BASE_URL)
    }

    // Project endpoints
    pub fn projects() -> String {
        format!("{}/api/projects", API_BASE_URL)
    }

    pub fn project_by_slug(slug: &str) -> String {
        format!("{}/api/projects/{}", API_BASE_URL, slug)
    }
}
