use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use mongodb::{bson, Database};
use uuid::Uuid;

use crate::routes::user::User;

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("Hashed password doesn't match.")]
    PasswordHashing(#[from] argon2::password_hash::Error),
    #[error("Thread error.")]
    ThreadError(#[from] anyhow::Error),
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub async fn validate_credentials(
    credentials: Credentials,
    db_client: &Database,
) -> Result<uuid::Uuid, AuthenticationError> {
    match db_client
        .collection::<User>("users")
        .find_one(bson::doc! {"username": credentials.username }, None)
        .await
        .unwrap_or(None)
    {
        Some(user) => {
            verify_password_hash(credentials.password, user.password_hash)?;
            return Ok(Uuid::parse_str(&user.uuid).expect("Failed parsing string to uuid."));
        }
        None => return Err(AuthenticationError::InvalidCredentials),
    }
}

fn verify_password_hash(
    current_password: String,
    hashed_password: String,
) -> Result<(), AuthenticationError> {
    let expected_password = PasswordHash::new(&hashed_password)?;
    Argon2::default().verify_password(&current_password.as_bytes(), &expected_password)?;
    Ok(())
}

pub async fn change_password(
    _current_password: String,
    _new_password: String,
) -> Result<(), AuthenticationError> {
    unimplemented!();
}

pub async fn create_hash_password(password: String) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();
    Ok(password_hash)
}
