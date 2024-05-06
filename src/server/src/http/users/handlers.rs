use axum::extract::State;
use axum::Json;
use crate::http::error::{HttpError, HttpResult};
use super::types::{
    NewUser, UpdateUser, User, UserBody
};
use super::util::{hash_password, verify_password};
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


/// Attempts to log in a user.
pub(super) async fn login_user(
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
pub(super) async fn get_current_user(
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


// Get the profile of a user.
// TODO: this should include game statistics and stuff; will handle that down the line
pub(super) async fn get_user_profile(
    app_state: State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<ProfileBody>> 
{
    unreachable!();

    // Since our query columns directly match an existing struct definition,
    // we can use `query_as!()` and save a bit of manual mapping.
    let profile = sqlx::query_as!(
        Profile,
        r#"
            select
                username,
                bio,
                image,
                exists(
                    select 1 from follow 
                    where followed_user_id = "user".user_id and following_user_id = $2
                ) "following!" -- This tells SQLx that this column will never be null
            from "user"
            where username = $1
        "#,
        username,
        maybe_auth_user.user_id()
    )
    .fetch_optional(&app_state.db)
    .await?
    .ok_or(HttpError::NotFound)?;

    Ok(Json(ProfileBody { profile }))
}


/// Updates a user.
/// Note from original author: Semantically this should be PATCH since it allows for partial updates
async fn update_user(
    app_state: State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<UserBody<UpdateUser>>,
) -> HttpResult<Json<UserBody<User>>> 
{
    if req.user == UpdateUser::default() {
        // If there's no fields to update, these two routes are effectively identical.
        return get_current_user(auth_user, app_state).await;
    }

    // WTB `Option::map_async()`
    let password_hash = if let Some(password) = req.user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query!(
        // This is how we do optional updates of fields without needing a separate query for each.
        // language=PostgreSQL
        r#"
            update "user"
            set email = coalesce($1, "user".email),
                username = coalesce($2, "user".username),
                password_hash = coalesce($3, "user".password_hash),
                bio = coalesce($4, "user".bio),
                image = coalesce($5, "user".image)
            where user_id = $6
            returning email, username, bio, image
        "#,
        req.user.email,
        req.user.username,
        password_hash,
        req.user.bio,
        req.user.image,
        auth_user.user_id
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
            email: user.email,
            token: auth_user.to_jwt(&app_state),
            username: user.username,
            bio: user.bio,
            image: user.image,
        },
    }))
}
