use axum::{
    routing::{delete, get, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::user::{
    change_password, change_role, delete_user, get_current_user, list_user, register_user,
};

pub fn build_user_router() -> Router<AppRegistry> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/password", put(change_password))
        .route("/users", get(list_user).post(register_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/users/:user_id/role", put(change_role))
}
