use axum::extract::Extension;
use axum::Json;
use crate::http::error::{HttpError, HttpResult};
use super::types::{
    UserBody,
    User,
    NewUser
};
use super::util::hash_password;
use super::AppState;

/// Creates a new user.
async fn create_user(
    ctx: Extension<AppState>,
    Json(req): Json<UserBody<NewUser>>,
) -> HttpResult<Json<UserBody<User>>> 
{
    let password_hash = hash_password(req.user.password).await?;
    let user_id = sqlx::query_scalar!(
        r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
        req.user.username,
        req.user.email,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        HttpError::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        HttpError::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(UserBody {
        user: User {
            email: req.user.email,
            token: AuthUser { user_id }.to_jwt(&ctx),
            username: req.user.username,
            bio: "".to_string(),
            image: None,
            coins: 0
        },
    }))
}