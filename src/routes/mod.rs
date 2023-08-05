use axum::{
    body::Body,
    routing::get,
    Router,
    response::IntoResponse,
    http::Response
};

mod activities;
mod nodeinfo;
mod objects;
mod users;
mod well_known;

// Hatsu & Version
async fn root() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");
    let message = format!("Hatsu\nVersion {}", version);

    Response::new(Body::from(message))
}

pub fn init() -> Router<(), Body> {
    Router::new()
        .merge(activities::init())
        .merge(nodeinfo::init())
        .merge(objects::init())
        .merge(users::init())
        .merge(well_known::init())
        .route("/", get(root))
}