use crate::handler::user::{
    change_password, change_role, delete_user, get_checkouts, get_current_user, list_users,
    register_user,
};
use axum::{
    routing::{delete, get, put},
    Router,
};
use registry::AppRegistry;

pub fn build_user_router() -> Router<AppRegistry> {
    let user_routers = Router::new()
        .route("/me", get(get_current_user))
        .route("/me/password", put(change_password))
        .route("/me/checkouts", get(get_checkouts))
        .route("/", get(list_users).post(register_user))
        .route("/:user_id", delete(delete_user))
        .route("/:user_id/role", put(change_role));

    Router::new().nest("/users", user_routers)
}
