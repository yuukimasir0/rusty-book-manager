use axum::{routing::post, Router};
use registry::AppRegistry;

use crate::handler::auth::{login, logout};

pub fn build_auth_routers() -> Router<AppRegistry> {
    let auth_routers = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));

    Router::new().nest("/auth", auth_routers)
}
