//! Single source of truth for route paths: registration, redirects, templates and tests use these.
pub const ROOT: &str = "/";
pub const DASHBOARD: &str = "/dashboard";

pub const LOGIN: &str = "/login";
pub const REGISTER: &str = "/register";
pub const LOGOUT: &str = "/logout";
pub const LOGOUT_ALL: &str = "/logout/all";

pub const API_LOGIN: &str = "/api/login";
pub const API_REGISTER: &str = "/api/register";
pub const API_LOGOUT: &str = "/api/logout";
pub const API_LOGOUT_ALL: &str = "/api/logout/all";
