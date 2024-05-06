use argon2::{password_hash::SaltString, Argon2, PasswordHash };
use crate::http::error::{HttpResult, HttpError};


/// Hashes a given password.
pub(super) async fn hash_password(password: String) -> HttpResult<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> HttpResult<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}

/// Verifies if a password corresponds to a hash.
pub(super) async fn verify_password(password: String, password_hash: String) -> HttpResult<()> {
    Ok(
        tokio::task::spawn_blocking(move || -> Result<()> {
            let hash = PasswordHash::new(&password_hash)
                .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

            hash.verify_password(&[&Argon2::default()], password)
                .map_err(|e| match e {
                    argon2::password_hash::Error::Password => HttpError::Unauthorized,
                    _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
                })
    }
    )
    .await
    .context("panic in verifying password hash")??)
}