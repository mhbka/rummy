use axum::extract::State;
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
pub(super) async fn create_user(
    app_state: State<AppState>,
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
    .fetch_one(&app_state.db)
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
            token: AuthUser { user_id }.to_jwt(&app_state),
            username: req.user.username,
            bio: "".to_string(),
            image: None,
            coins: 0
        },
    }))
}

// Attempts to log in a user.
async fn login_user(
    app_state: State<AppState>,
    Json(req): Json<UserBody<LoginUser>>,
) -> HttpResult<Json<UserBody<User>>> {
    let user = sqlx::query!(
        r#"
            select user_id, email, username, bio, image, password_hash 
            from "user" where email = $1
        "#,
        req.user.email,
    )
    .fetch_optional(&app_state.db)
    .await?
    .ok_or(HttpError::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: AuthUser {
                user_id: user.user_id,
            }
            .to_jwt(&app_state),
            username: user.username,
            bio: user.bio,
            image: user.image,
            coins: user.coins
        },
    }))
}

/// Gets the current user.
async fn get_current_user(
    app_state: State<AppState>,
    auth_user: AuthUser,
) -> HttpResult<Json<UserBody<User>>> 
{
    let user = sqlx::query!(
        r#"select email, username, bio, image from "user" where user_id = $1"#,
        auth_user.user_id
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            // The spec doesn't state whether we're supposed to return the same token we were passed,
            // or generate a new one. Generating a new one is easier the way the code is structured.
            //
            // This has the side-effect of automatically refreshing the session if the frontend
            // updates its token based on this response.
            token: auth_user.to_jwt(&app_state),
            username: user.username,
            bio: user.bio,
            image: user.image,
            coins: user.coins
        },
    }))
}

