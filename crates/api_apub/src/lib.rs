use axum::Router;

mod activities;
mod posts;
mod users;

pub fn routes() -> Router {
    Router::new()
        .merge(activities::routes())
        .merge(posts::routes())
        .merge(users::routes())
}
