use axum::Router;
use axum::routing::{get, post};
use super::super::AppState;
use super::handlers::{
    create_user,
    login_user,
    get_current_user,
    update_user,
    get_user_profile
};

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(create_user))
        .route("/login", post(login_user))
        .route("/update", get(get_current_user).put(update_user))
        .route("/profiles/:username", get(get_user_profile)) 
}