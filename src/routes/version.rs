use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
struct Git<'a> {
    repo: &'a str,
    commit: &'a str,
}

#[derive(Serialize)]
pub struct Version<'a> {
    version: &'a str,
    git: Git<'a>,
}

#[get("/version")]
pub fn version() -> Json<Version<'static>> {
    let version = Version {
        version: env!("CARGO_PKG_VERSION"),
        git: Git {
            repo: env!("CARGO_PKG_REPOSITORY"),
            commit: env!("GIT_HASH"),
        },
    };
    Json(version)
}
