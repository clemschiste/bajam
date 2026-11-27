//! Single source of truth for route paths: registration, redirects, templates and tests use these.
//! `{name}` segments are path parameters; fill them with the builder functions below.
use std::fmt::Display;

pub const ROOT: &str = "/";
pub const DASHBOARD: &str = "/dashboard";
pub const STATIC: &str = "/static";

pub const LOGIN: &str = "/login";
pub const REGISTER: &str = "/register";
pub const LOGOUT: &str = "/logout";
pub const LOGOUT_ALL: &str = "/logout/all";

pub const ESCAPADES: &str = "/escapades";
pub const ESCAPADE: &str = "/escapades/{escapade_id}";
pub const ESCAPADE_HEARTBEAT: &str = "/escapades/{escapade_id}/heartbeat";
pub const ESCAPADE_HEARTBEATS: &str = "/escapades/{escapade_id}/heartbeats";
pub const HEARTBEAT: &str = "/heartbeats/{heartbeat_id}";
pub const UPLOAD: &str = "/upload/{picture_id}";
pub const IMAGE: &str = "/image/{picture_id}";

pub const API_LOGIN: &str = "/api/login";
pub const API_REGISTER: &str = "/api/register";
pub const API_LOGOUT: &str = "/api/logout";
pub const API_LOGOUT_ALL: &str = "/api/logout/all";
pub const API_ESCAPADES: &str = "/api/escapades";
pub const API_ESCAPADE: &str = "/api/escapades/{escapade_id}";
pub const API_ESCAPADE_HEARTBEAT: &str = "/api/escapades/{escapade_id}/heartbeat";

fn fill(pattern: &str, name: &str, value: impl Display) -> String {
    pattern.replace(&format!("{{{name}}}"), &value.to_string())
}

pub fn escapade(id: impl Display) -> String {
    fill(ESCAPADE, "escapade_id", id)
}

pub fn escapade_heartbeat(id: impl Display) -> String {
    fill(ESCAPADE_HEARTBEAT, "escapade_id", id)
}

pub fn escapade_heartbeats(id: impl Display) -> String {
    fill(ESCAPADE_HEARTBEATS, "escapade_id", id)
}

pub fn heartbeat(id: impl Display) -> String {
    fill(HEARTBEAT, "heartbeat_id", id)
}

pub fn upload(id: impl Display) -> String {
    fill(UPLOAD, "picture_id", id)
}

pub fn image(id: impl Display) -> String {
    fill(IMAGE, "picture_id", id)
}

pub fn api_escapade(id: impl Display) -> String {
    fill(API_ESCAPADE, "escapade_id", id)
}

pub fn api_escapade_heartbeat(id: impl Display) -> String {
    fill(API_ESCAPADE_HEARTBEAT, "escapade_id", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_fills_parameter() {
        assert_eq!(escapade_heartbeats(42), "/escapades/42/heartbeats");
    }
}
