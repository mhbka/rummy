
use crate::http::error::{HttpError, HttpResult};
use crate::http::AppState;
use axum::body::Body;
use axum::extract::{Extension, FromRef, FromRequest, FromRequestParts, RequestParts};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::HeaderValue;
use async_trait::async_trait;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha384;
use time::OffsetDateTime;
use uuid::Uuid;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);

// Ideally the Realworld spec would use the `Bearer` scheme as that's relatively standard
// and has parsers available, but it's really not that hard to parse anyway.
const SCHEME_PREFIX: &str = "Token ";

/// Add this as a parameter to a handler function to require the user to be logged in.
///
/// Parses a JWT from the `Authorization: Token <token>` header.
pub struct AuthUser {
    pub user_id: Uuid,
}

/// Add this as a parameter to a handler function to optionally check if the user is logged in.
///
/// If the `Authorization` header is absent then this will be `Self(None)`, otherwise it will
/// validate the token.
///
/// This is in contrast to directly using `Option<AuthUser>`, which will be `None` if there
/// is *any* HttpError in deserializing, which isn't exactly what we want.
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    /// Standard JWT `exp` claim.
    exp: i64,
}

impl AuthUser {
    pub(in crate::http) fn to_jwt(&self, state: &AppState) -> String {
        let hmac = Hmac::<Sha384>::new_from_slice(state.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        AuthUserClaims {
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }

    /// Attempt to parse `AuthUser` from an `Authorization` header.
    fn from_authorization(state: &AppState, auth_header: &HeaderValue) -> HttpResult<Self> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::debug!("Authorization header is not UTF-8");
            HttpError::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!(
                "Authorization header is using the wrong scheme: {:?}",
                auth_header
            );
            return Err(HttpError::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt =
            jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
                log::debug!(
                    "failed to parse Authorization header {:?}: {}",
                    auth_header,
                    e
                );
                HttpError::Unauthorized
            })?;

        // Realworld doesn't specify the signing algorithm for use with the JWT tokens
        // so we picked SHA-384 (HS-384) as the HMAC, as it is more difficult to brute-force
        // than SHA-256 (recommended by the JWT spec) at the cost of a slightly larger token.
        let hmac = Hmac::<Sha384>::new_from_slice(state.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        // When choosing a JWT implementation, be sure to check that it validates that the signing
        // algorithm declared in the token matches the signing algorithm you're verifying with.
        // The `jwt` crate does.
        let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
            log::debug!("JWT failed to verify: {}", e);
            HttpError::Unauthorized
        })?;

        let (_header, claims) = jwt.into();

        // Because JWTs are stateless, we don't really have any mechanism here to invalidate them
        // besides expiration. You probably want to add more checks, like ensuring the user ID
        // exists and has not been deleted/banned/deactivated.
        //
        // You could also use the user's password hash as part of the keying material for the HMAC,
        // so changing their password invalidates their existing sessions.
        //
        // In practice, Launchbadge has abandoned using JWTs for authenticating long-lived sessions,
        // instead storing session data in Redis, which can be accessed quickly and so adds less
        // overhead to every request compared to hitting Postgres, and allows tracking and
        // invalidating individual sessions by simply deleting them from Redis.
        //
        // Technically, the Realworld spec isn't all that adamant about using JWTs and there
        // may be some flexibility in using other kinds of tokens, depending on whether the frontend
        // is expected to parse the token or just treat it as an opaque string.
        //
        // Also, if the consumer of your API is a browser, you probably want to put your session
        // token in a cookie instead of the response body. By setting the `HttpOnly` flag, the cookie
        // isn't exposed in the response to Javascript at all which, along with setting `Domain` and
        // `SameSite`, prevents all kinds of session hijacking exploits.
        //
        // This also has the benefit of avoiding having to deal with securely storing the session
        // token on the frontend.

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("token expired");
            return Err(HttpError::Unauthorized);
        }

        Ok(Self {
            user_id: claims.user_id,
        })
    }
}

impl MaybeAuthUser {
    /// If this is `Self(Some(AuthUser))`, return `AuthUser::user_id`
    pub fn user_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|auth_user| auth_user.user_id)
    }
}

// tower-http has a `RequireAuthorizationLayer` but it's useless for practical applications,
// as it only supports matching Basic or Bearer auth with credentials you provide it.
//
// There's the `::custom()` constructor to provide your own validator but it basically
// requires parsing the `Authorization` header by-hand anyway so you really don't get anything
// out of it that you couldn't write your own middleware for, except with a bunch of extra
// boilerplate.
#[async_trait]
impl FromRequest for AuthUser {
    type Rejection = HttpError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let state: Extension<AppState> = Extension::from_request(req)
            .await
            .expect("BUG: AppState was not added as an extension");

        // Get the value of the `Authorization` header, if it was sent at all.
        let auth_header = req
            .headers()
            .ok_or(HttpError::Unauthorized)?
            .get(AUTHORIZATION)
            .ok_or(HttpError::Unauthorized)?;

        Self::from_authorization(&state, auth_header)
    }
}

// tower-http has a `RequireAuthorizationLayer` but it's useless for practical applications,
// as it only supports matching Basic or Bearer auth with credentials you provide it.
//
// There's the `::custom()` constructor to provide your own validator but it basically
// requires parsing the `Authorization` header by-hand anyway so you really don't get anything
// out of it that you couldn't write your own middleware for, except with a bunch of extra
// boilerplate.
#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: AppState = AppState::from_ref(state);

        // Get the value of the `Authorization` header, if it was sent at all.
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(HttpError::Unauthorized)?;

        Self::from_authorization(&ctx, auth_header)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MaybeAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: AppState = AppState::from_ref(state);

        Ok(Self(
            // Get the value of the `Authorization` header, if it was sent at all.
            parts
                .headers
                .get(AUTHORIZATION)
                .map(|auth_header| AuthUser::from_authorization(&ctx, auth_header))
                .transpose()?,
        ))
    }
}