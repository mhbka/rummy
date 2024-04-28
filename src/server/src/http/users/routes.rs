use axum::Router;

pub(crate) fn router() -> Router<ApiContext> {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/users/signup", post(create_user))
        .route("/users/login", post(login_user))
        .route("/users/update", get(get_current_user).put(update_user))
}